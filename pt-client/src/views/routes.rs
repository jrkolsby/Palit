use std::io::{Write, Stdout};
use std::collections::HashMap;
use termion::cursor;
use xmltree::Element;

use crate::common::{MultiFocus, FocusType, ID, VOID_ID};
use crate::common::{shift_focus, render_focii, focus_dispatch};
use crate::common::{Screen, Action, Window, Anchor, Route};
use crate::common::{write_fg, write_bg};
use crate::views::{Layer};
use crate::components::{button, popup};
use crate::modules::patch;

#[derive(Clone, Debug)]
pub struct RoutesState {
    pub routes: HashMap<u16, Route>,
    pub anchors: HashMap<u16, Anchor>,
    pub focus: (usize, usize),
    pub selected_route: Option<u16>,
    pub selected_anchor: Option<u16>,
}

pub struct Routes {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: RoutesState,
    focii: Vec<Vec<MultiFocus<RoutesState>>>
}

static PADDING: (u16, u16) = (3,3);

static VOID_RENDER: fn( &mut Screen, Window, ID, &RoutesState, bool) =
    |_, _, _, _, _| {};

fn generate_focii(
    routes: &HashMap<u16, Route>, 
    anchors: &HashMap<u16, Anchor>
) -> Vec<Vec<MultiFocus::<RoutesState>>> {
    let void_focus = MultiFocus::<RoutesState> {
        w_id: (FocusType::Void, 0),
        w: VOID_RENDER,
        r_id: (FocusType::Void, 0),
        r: VOID_RENDER,
        r_t: |_, _, _| Action::Noop,
        g_id: (FocusType::Void, 0), 
        g: VOID_RENDER,
        g_t: |_, _, _| Action::Noop,
        p_id:(FocusType::Void, 0), 
        p: VOID_RENDER,
        p_t: |_, _, _| Action::Noop,
        y_id:(FocusType::Void, 0), 
        y: VOID_RENDER,
        y_t: |_, _, _| Action::Noop,
        b_id: (FocusType::Void, 0), 
        b: VOID_RENDER,
        b_t: |_, _, _| Action::Noop,
        active: None,
    };
    let mut focii = vec![];

    let mut counter = 0;
    let mut focus_acc = void_focus.clone();

    let mut sorted_anchors: Vec<Anchor> = anchors.iter()
        .map(|(_, a)| a.clone()).collect::<Vec<Anchor>>();

    sorted_anchors.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());

    for anchor in sorted_anchors.iter() {
        let id = (FocusType::Button, anchor.index);
        let transform: fn(Action, ID, &RoutesState) -> Action = |a, id, state| match a {
            // Change selected route
            Action::Left => if let Some(id) = state.selected_route {
                if state.routes.contains_key(&(id-1)) {
                    Action::PatchRoute(id-1)        // Move patch
                } else { Action::PatchRoute(id) }   // Remove patch
            } else { Action::PatchRoute(1) },       // Patch to master
            Action::Right => if let Some(id) = state.selected_route {
                if state.routes.contains_key(&(id+1)) {
                    Action::PatchRoute(id+1)        // Move patch
                } else { Action::PatchRoute(id) }   // Remove patch
            } else { Action::PatchRoute(1) },       // Patch to master
            // Or set selected anchor
            Action::SelectR |
            Action::SelectG |
            Action::SelectP |
            Action::SelectY |
            Action::SelectB => Action::PatchAnchor(id.1),
            _ => Action::Noop
        };
        let render: fn(&mut Screen, Window, ID, &RoutesState, bool) = 
            |mut out, window, id, state, focus| {
                let anchor = &state.anchors.get(&id.1).unwrap();
                write!(out, "{}{} {}", cursor::Goto(
                    (PADDING.0 * 2) + window.x + 2 * state.routes.len() as u16,
                    (PADDING.1) + window.y + anchor.index * 2
                ), match anchor.input {
                    true =>  "─▶",
                    false =>  "◀◀",
                }, anchor.name.clone()).unwrap();
        };
        counter = match counter {
            0 => { focus_acc.r_id = id; focus_acc.r = render; focus_acc.r_t = transform; 1 },
            1 => { focus_acc.g_id = id; focus_acc.g = render; focus_acc.g_t = transform; 2 },
            2 => { focus_acc.p_id = id; focus_acc.p = render; focus_acc.p_t = transform; 3 },
            3 => { focus_acc.y_id = id; focus_acc.y = render; focus_acc.y_t = transform; 4 },
            _ => { focus_acc.b_id = id; focus_acc.b = render; focus_acc.b_t = transform; 0 },
        };
        if counter == 0 { 
            focii.push(vec![focus_acc]);
            focus_acc = void_focus.clone();
        }
    }
    if counter > 0 { focii.push(vec![focus_acc]); }

    let mut footer_options = void_focus.clone();

    footer_options.r_id = (FocusType::Button, 0);
    footer_options.r = |out, win, id, state, focus| {
        write!(out, "{}Add Route", cursor::Goto(
            PADDING.0 + win.x,
            win.y + win.h - PADDING.1)
        ).unwrap();
    };
    footer_options.r_t = |a, id, state| {
        match a { Action::SelectR => {
            let mut new_id = state.routes.iter().fold(0, |max, (_,r)| 
                if r.id > max {r.id} else {max}) + 1;
            Action::AddRoute(new_id)
        }, _ => Action::Noop}
    };
    footer_options.y_id = (FocusType::Button, 0);
    footer_options.y = |out, win, id, state, focus| {
        write!(out, "{}Delete Route", cursor::Goto(
            PADDING.0 + 11 + win.x,
            win.y + win.h - PADDING.1)
        ).unwrap();
    };
    footer_options.y_t = |a, id, state| {
        match a { Action::SelectY => {
            let mut del_id = state.routes.iter().fold(0, |max, (_,r)|
                if r.id > max {r.id} else {max});
            Action::DelRoute(del_id)
        }, _ => Action::Noop}
    };

    focii.push(vec![footer_options]);

    focii
}

fn reduce(state: RoutesState, action: Action) -> RoutesState {
    RoutesState {
        routes: match action {
            Action::AddRoute(a) => {
                let mut new_routes = state.routes.clone();
                new_routes.insert(a, Route {
                    id: a,
                    patch: vec![]
                });
                new_routes
            },
            Action::PatchOut(m_id, a_id, r_id) |
            Action::PatchIn(m_id, a_id, r_id) => {
                let mut new_routes = state.routes.clone();
                let route = new_routes.get_mut(&r_id).unwrap();
                let anchor = state.anchors.get(&a_id).unwrap();
                route.patch.push(anchor.clone());
                new_routes
            },
            Action::PatchAnchor(a_id) |
            Action::DelPatch(_, a_id, _) => {
                let mut new_routes = state.routes.clone();
                let module_id = state.anchors.get(&a_id).unwrap().module_id;
                'search: for (_, route) in new_routes.iter_mut() {
                    for (i, anchor) in route.patch.iter().enumerate() {
                        if anchor.index == a_id && anchor.module_id == module_id {
                            route.patch.remove(i);
                            break 'search;
                        }
                    }
                }
                new_routes
            },
            Action::DelRoute(r_id) => {
                let mut new_routes = state.routes.clone();
                new_routes.remove(&r_id);
                new_routes
            },
            _ => state.routes.clone()
        },
        focus: state.focus,
        selected_anchor: match action {
            Action::PatchAnchor(id) => {
                if let Some(_id) = state.selected_anchor {
                    if _id == id { None } 
                    else { Some(id) }
                } else {
                    Some(id)
                }
            },
            Action::DelPatch(_,_,_) |
            Action::PatchIn(_,_,_) |
            Action::PatchOut(_,_,_) => None,
            _ => state.selected_anchor.clone()
        },
        selected_route: match action {
            Action::PatchRoute(id) => {
                if let Some(_id) = state.selected_route {
                    if _id == id { None } 
                    else { Some(id) }
                } else {
                    Some(id)
                }
            },
            Action::DelPatch(_,_,_) |
            Action::PatchIn(_,_,_) |
            Action::PatchOut(_,_,_) => None,
            _ => state.selected_route.clone()
        },
        anchors: match action {
            Action::ShowAnchors(a) => {
                let mut new_anchors = HashMap::new();
                for anchor in a {
                    new_anchors.insert(anchor.index.clone(), anchor);
                }
                new_anchors
            },
            _ => state.anchors.clone()
        },
    }
}

impl Routes {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Option<Element>) -> Self {

        // Initialize State
        let mut initial_state: RoutesState = if let Some(el) = doc {
            patch::read(el)
        } else { RoutesState {
            routes: HashMap::new(),
            anchors: HashMap::new(),
            focus: (0,0),
            selected_anchor: None,
            selected_route: None,
        }};

        Routes {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![]],
        }
    }
}

fn render_patch(out: &mut Screen, 
    a: &Anchor, 
    r_id: u16, 
    anchor_pos: (u16, u16), 
    win: Window) {
    for x in (win.x + 2 * r_id)..anchor_pos.0 {
        write!(out, "{}─", cursor::Goto(
            PADDING.0 + x, 
            anchor_pos.1
        )).unwrap();
    }
    write!(out, "{}├", cursor::Goto(
        PADDING.0 + win.x + 2 * r_id - 1, 
        anchor_pos.1
    )).unwrap();
}

impl Layer for Routes {
    fn render(&self, out: &mut Screen, target: bool) {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        popup::render(out, win.x, win.y, win.w, win.h, &"Patch".to_string());

        render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, true, !target);

        let anchor_x = win.x + 2 * self.state.routes.len() as u16 + PADDING.0;

        let mut sorted_routes: Vec<Route> = self.state.routes.iter()
            .map(|(_, a)| a.clone()).collect::<Vec<Route>>();

        sorted_routes.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

        for route in sorted_routes {
            write!(out, "{}{}", cursor::Goto(
                PADDING.0 + win.x + route.id * 2 - 1,  
                PADDING.1 + win.y - 1,
            ), match route.id {
                1 => "M".to_string(),
                n => format!("{}", n)
            }).unwrap();

            // Draw vertical line
            for y in 0..(win.h - PADDING.1 * 2 - 1) {
                write!(out, "{}│", cursor::Goto(
                    PADDING.0 + win.x + route.id * 2 - 1, 
                    PADDING.1 + win.y + y)
                ).unwrap();
            }

            for anchor in route.patch.iter() {
                let anchor_y = PADDING.1 + win.y + (anchor.index * 2);
                if let Some(a) = self.state.anchors.get(&anchor.index) {
                    if a.module_id == anchor.module_id {
                        render_patch(out, &anchor, route.id, (anchor_x, anchor_y), win);
                    }
                }
            }
        }

        if let Some(a_id) = self.state.selected_anchor {
            let anchor_y = PADDING.1 + win.y + (a_id * 2);
            let anchor = self.state.anchors.get(&a_id).unwrap();
            if let Some(r_id) = self.state.selected_route {
                render_patch(out, &anchor, r_id, (anchor_x, anchor_y), win);
            }
        }
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Intercept arrow actions to change focus
        let (focus, default) = focus_dispatch(self.state.focus, 
                                              &mut self.focii, 
                                              &self.state, 
                                              action.clone());
        self.state.focus = focus;

        // Set focus, if the multifocus defaults, take no further action
        if let Some(_default) = default {
            let filtered_action = match _default {
                // On keyup we should patch to the selected route or disconnect this patch
                Action::Deselect => {
                    if let Some(a_id) = self.state.selected_anchor {
                        let anchor = self.state.anchors.get(&a_id).unwrap();
                        if let Some(r_id) = self.state.selected_route {
                            if anchor.input {
                                Action::PatchIn(
                                    anchor.module_id,
                                    anchor.index,
                                    r_id
                                )
                            } else {
                                Action::PatchOut(
                                    anchor.module_id,
                                    anchor.index,
                                    r_id
                                )
                            }
                        } else { 
                            Action::DelPatch(
                                anchor.module_id,
                                anchor.index,
                                anchor.input
                            )
                        }
                    } else { Action::Noop }
                },
                a => a
            };
            self.state = reduce(self.state.clone(), filtered_action.clone());
            match filtered_action {
                // Should delete existing connection on keydown
                Action::PatchAnchor(id) => {
                    let anchor = &self.state.anchors[&id];
                    Action::DelPatch(
                        anchor.module_id, 
                        anchor.index, 
                        anchor.input
                    )
                },
                a @ Action::Exit |
                a @ Action::Up | 
                a @ Action::Down => {
                    // About to change modules, reset selects
                    self.state.selected_anchor = None;
                    self.state.selected_route = None;
                    self.state.focus = (0,0);
                    return a;
                }
                Action::ShowAnchors(_) => {
                    self.focii = generate_focii(&self.state.routes, &self.state.anchors);
                    Action::Noop
                },
                a => a
            }
        } else { Action::Noop }
    }
    fn undo(&mut self) {
        self.state = self.state.clone()
    }
    fn redo(&mut self) {
        self.state = self.state.clone()
    }
    fn alpha(&self) -> bool {
        true
    }
}
