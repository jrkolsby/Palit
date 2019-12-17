use std::io::{Write, Stdout};
use std::collections::HashMap;
use termion::{color, cursor};
use termion::raw::{RawTerminal};
use xmltree::Element;

use crate::common::{MultiFocus, FocusType, ID, VOID_ID};
use crate::common::{shift_focus, render_focii, focus_dispatch};
use crate::common::{Action, Window, Anchor};
use crate::views::{Layer};
use crate::components::{button, bigtext};

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
    let mut focii = vec![vec![]];

    let mut counter = 0;
    let mut focus_acc = void_focus.clone();

    for (_, route) in routes.iter() {
        let id = (FocusType::Button, route.id);
        let transform: fn(Action, ID, &RoutesState) -> Action = |a, id, _| match a {
            Action::SelectR |
            Action::SelectG |
            Action::SelectP |
            Action::SelectY |
            Action::SelectB => Action::PatchRoute(id.1),
            _ => Action::Noop
        };
        let render: fn( RawTerminal<Stdout>, Window, ID, &RoutesState, bool) 
            -> RawTerminal<Stdout> = |mut out, window, id, state, focus| {
            write!(out, "{}{}", cursor::Goto(window.x + id.1, window.y + id.1 - 1), match id.1 {
                1 => "MASTER".to_string(),
                n => format!("ROUTE {}", n)
            });
            for y in 0..window.h {
                write!(out, "{}│", cursor::Goto(window.x + id.1 - 1, window.y + y));
            }
            if let Some(_id) = state.selected_route {
                if _id == id.1 {
                    write!(out, "{}^", cursor::Goto(window.x + id.1 - 1, window.y + window.h));
                }
            }
            out
        };
        counter = match counter {
            0 => { focus_acc.r_id = id; focus_acc.r = render; focus_acc.r_t = transform; 1 },
            1 => { focus_acc.g_id = id; focus_acc.g = render; focus_acc.g_t = transform; 2 },
            2 => { focus_acc.p_id = id; focus_acc.p = render; focus_acc.p_t = transform; 3 },
            3 => { focus_acc.y_id = id; focus_acc.y = render; focus_acc.y_t = transform; 4 },
            _ => { focus_acc.b_id = id; focus_acc.b = render; focus_acc.b_t = transform; 5 },
        };
        if counter == 0 { 
            focii[0].push(focus_acc); 
            focus_acc = void_focus.clone();
        }
    }
    if counter > 0 { focii[0].push(focus_acc); }

    focus_acc = void_focus.clone();
    counter = 0;

    for (_, anchor) in anchors.iter() {
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
            write!(out, "{}{} {}", 
                cursor::Goto(window.x+anchor.x, window.y+anchor.y), 
                match anchor.input {
                    true => if focus { "─▶" } else { "" },
                    false => if focus { "◀─" } else { "" }, 
                }, anchor.name.clone());
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
            focii[0].push(focus_acc); 
            focus_acc = void_focus.clone();
        }
    }
    if counter > 0 { focii[0].push(focus_acc); }
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
            }
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
            }
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

impl Layer for Routes {
    fn render(&self, mut out: RawTerminal<Stdout>, target: bool) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, !target);

        if let Some(a_id) = self.state.selected_anchor {
            let anchor = self.state.anchors.get(&a_id).unwrap();
            if let Some(r_id) = self.state.selected_route {
                for x in (win.x + r_id)..(win.x + anchor.x) {
                    write!(out, "{}─", cursor::Goto(x, win.y+anchor.y));
                }
                write!(out, "{}├", cursor::Goto(win.x+r_id-1, win.y+anchor.y));
            } else {
                // Draw stem to anchor
                let mut end = true;
                for x in (win.x + anchor.x - 5)..(win.x + anchor.x) {
                    write!(out, "{}{}", cursor::Goto(x, win.y+anchor.y), match end {
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
            self.state = reduce(self.state.clone(), _default.clone());
            match _default {
                Action::Exit |
                Action::Up | Action::Down => {
                    // About to change modules, reset selects
                    self.state.selected_anchor = None;
                    self.state.selected_route = None;
                    return _default;
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
    fn shift(&mut self, x: u16, y: u16) {}
}
