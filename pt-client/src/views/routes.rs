use std::io::{Write, Stdout};
use std::collections::HashMap;
use termion::{color, cursor};
use termion::raw::{RawTerminal};
use xmltree::Element;

use crate::common::{MultiFocus, FocusType, ID, VOID_ID};
use crate::common::{shift_focus, render_focii, focus_dispatch};
use crate::common::{Action, Window, Anchor, Color};
use crate::common::{write_fg, write_bg};
use crate::views::{Layer};
use crate::components::{button, popup};

#[derive(Clone, Debug)]
struct Route {
    id: u16,
    patch: Vec<Anchor>,
}

#[derive(Clone, Debug)]
pub struct RoutesState {
    routes: HashMap<u16, Route>,
    anchors: HashMap<u16, Anchor>,
    focus: (usize, usize),
    selected_route: Option<u16>,
    selected_anchor: Option<u16>,
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

static VOID_RENDER: fn( RawTerminal<Stdout>, 
        Window, ID, &RoutesState, bool) -> RawTerminal<Stdout> =
    |mut out, window, id, state, focus| out;

fn generate_focii(
    routes: &HashMap<u16, Route>, 
    anchors: &HashMap<u16, Anchor>
) -> Vec<Vec<MultiFocus::<RoutesState>>> {
    let void_focus = MultiFocus::<RoutesState> {
        w_id: (FocusType::Void, 0),
        w: VOID_RENDER,
        r_id: (FocusType::Void, 0),
        r: |out, _, _, _, _| out,
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

    sorted_anchors.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for anchor in sorted_anchors.iter() {
        let id = (FocusType::Button, anchor.id);
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
        let render: fn( RawTerminal<Stdout>, Window, ID, &RoutesState, bool
            ) -> RawTerminal<Stdout> = |mut out, window, id, state, focus| {
            let anchor = &state.anchors.get(&id.1).unwrap();
            if !focus { out = write_bg(out, Color::Beige); out = write_fg(out, Color::Black); }
            write!(out, "{}{} {}", cursor::Goto(
                    (PADDING.0 * 2) + window.x + state.routes.len() as u16,
                    (PADDING.1 * 2) + window.y + anchor.id * 2
            ), match anchor.input {
                    true =>  "─▶",
                    false =>  "◀─",
                }, anchor.name.clone()
            );
            out
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
            Action::PatchIn(_,_,_) |
            Action::PatchOut(_,_,_) => None,
            _ => state.selected_route.clone()
        },
        anchors: match action {
            Action::ShowAnchors(a) => {
                let mut new_anchors = HashMap::new();
                for anchor in a {
                    new_anchors.insert(anchor.id.clone(), anchor);
                }
                new_anchors
            },
            _ => state.anchors.clone()
        },
    }
}

impl Routes {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Option<Element>) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let mut initial_state: RoutesState = RoutesState {
            routes: HashMap::new(),
            anchors: HashMap::new(),
            focus: (0,0),
            selected_anchor: None,
            selected_route: None,
        };
        
        initial_state.routes.insert(1, Route { id: 1, patch: vec![] });
        initial_state.routes.insert(2, Route { id: 2, patch: vec![] });

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

fn render_patch(mut out: RawTerminal<Stdout>, 
    a: &Anchor, 
    r_id: u16, 
    anchor_pos: (u16, u16), 
    win: Window) -> RawTerminal<Stdout> {
    for x in (win.x + r_id)..anchor_pos.0 {
        write!(out, "{}─", cursor::Goto(
            PADDING.0 + x, anchor_pos.1
        ));
    }
    write!(out, "{}├", cursor::Goto(
        PADDING.0 + win.x + r_id - 1, anchor_pos.1
    ));
    out
}

impl Layer for Routes {
    fn render(&self, mut out: RawTerminal<Stdout>, target: bool) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = popup::render(out, win.x, win.y, win.w, win.h, &"Routes".to_string());

        out = render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, !target);

        out = write_bg(out, Color::Beige); 
        out = write_fg(out, Color::Black);

        let anchor_x = win.x + self.state.routes.len() as u16 + PADDING.0;

        for (_, route) in self.state.routes.iter() {
            write!(out, "{}{}", cursor::Goto(
                PADDING.0 + win.x + route.id, 
                PADDING.1 + win.y + route.id - 1
            ), match route.id {
                1 => "MASTER".to_string(),
                n => format!("ROUTE {}", n)
            });

            // Draw vertical line
            for y in 0..(win.h - PADDING.1 * 2) {
                write!(out, "{}│", cursor::Goto(
                    PADDING.0 + win.x + route.id - 1, 
                    PADDING.1 + win.y + y)
                );
            }

            // Draw route selector
            if let Some(_id) = self.state.selected_route {
                if _id == route.id {
                    write!(out, "{}^", cursor::Goto(
                        PADDING.0 + win.x + route.id - 1, 
                        win.y + (win.h - PADDING.1)
                    ));
                }
            }

            for anchor in route.patch.iter() {
                let anchor_y = win.y + (anchor.id * 2) + (PADDING.1 * 2);
                out = render_patch(out, &anchor, route.id, (anchor_x, anchor_y), win);


            }
        }

        if let Some(a_id) = self.state.selected_anchor {
            let anchor_y = win.y + (a_id * 2) + (PADDING.1 * 2);
            let anchor = self.state.anchors.get(&a_id).unwrap();
            if let Some(r_id) = self.state.selected_route {
                out = render_patch(out, &anchor, r_id, (anchor_x, anchor_y), win);
            } else {
                // Draw stem to anchor
                let mut end = true;
                for x in (win.x + anchor_x - 5)..anchor_x {
                    write!(out, "{}{}", cursor::Goto(
                        PADDING.0 + x, anchor_y
                    ), match end {
                        true => "?",
                        false => "─"
                    });
                    end = false;
                }
            }
        }

        out.flush().unwrap();
        out
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
                Action::Deselect => {
                    if let Some(a_id) = self.state.selected_anchor {
                        if let Some(r_id) = self.state.selected_route {
                            let anchor = self.state.anchors.get(&a_id).unwrap();
                            if anchor.input {
                                Action::PatchIn(
                                    anchor.module_id,
                                    anchor.id,
                                    r_id
                                )
                            } else {
                                Action::PatchOut(
                                    anchor.module_id,
                                    anchor.id,
                                    r_id
                                )
                            }
                        } else { Action::Noop }
                    } else { Action::Noop }
                },
                a => a
            };
            self.state = reduce(self.state.clone(), filtered_action.clone());
            match filtered_action {
                a @ Action::Exit |
                a @ Action::Up | a @ Action::Down => {
                    // About to change modules, reset selects
                    self.state.selected_anchor = None;
                    self.state.selected_route = None;
                    self.state.focus = (0,0);
                    return a;
                }
                Action::ShowAnchors(_) |
                Action::AddRoute(_) =>  {
                    self.focii = generate_focii(&self.state.routes, &self.state.anchors);
                    Action::CountRoutes(self.state.routes.len() as u16)
                },
                _ => { Action::Noop }
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
