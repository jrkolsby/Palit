use std::io::{Write, Stdout};
use std::collections::HashMap;
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, MultiFocus, shift_focus, render_focii, FocusType, Window};
use crate::views::{Layer};
use crate::components::{button};

#[derive(Clone, Debug)]
pub struct RoutesState {
    routes: HashMap<u16, (Vec<u16>, Vec<u16>)>, // id, in ids, out ids
    ins: HashMap<u16, (u16, u16, u16)>, // id, layer_id, x, y
    outs: HashMap<u16, (u16, u16, u16)>, // ^
    focus: (usize, usize),
}

pub struct Routes {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: RoutesState,
    focii: Vec<Vec<MultiFocus<RoutesState>>>
}

type Route = (u16, Vec<u16>); // id, vector of input or output ids

fn reduce(state: RoutesState, action: Action) -> RoutesState {
    RoutesState {
        routes: match action {
            Action::Patch(a,b,c) => {
                let mut new_routes = state.routes.clone();
                if let Some(entry) = new_routes.get_mut(&a) {
                    entry.0.push(b);
                    entry.1.push(c);
                } else {
                    new_routes.insert(a, (vec![b], vec![c]));
                }
                new_routes
            },
            _ => state.routes.clone()
        },
        ins: state.ins.clone(),
        outs: state.outs.clone(),
        focus: state.focus,
    }
}

impl Routes {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: RoutesState = RoutesState {
            routes: HashMap::new(),
            ins: HashMap::new(),
            outs: HashMap::new(),
            focus: (0,0),
        };

        Routes {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<RoutesState> {
                    r: |mut out, window, state| {
                        button::render(out, window.x+2, window.y+2, 20, "Add Route")
                    },
                    r_t: |action, id, state| {  
                        match action {
                            Action::SelectR => Action::Patch(0,0,0),
                            _ => Action::Noop
                        }
                    },
                    r_id: (FocusType::Button, 0),
                    g: |mut out, window, state| out,
                    g_t: |action, id, state| action,
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, state| out,
                    y_t: |action, id, state| action,
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, state| out,
                    p_t: |action, id, state| action,
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, state| out,
                    b_t: |action, id, state| action,
                    b_id: (FocusType::Button, 0),
                    active: None,
                }

            ]],
        }
    }
}

impl Layer for Routes {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = render_focii(out, win, self.state.focus.clone(), &self.focii, &self.state);

        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Let the focus transform the action 
        let multi_focus = &mut self.focii[self.state.focus.1][self.state.focus.0];
        let _action = multi_focus.transform(action.clone(), &mut self.state);

        // Intercept arrow actions to change focus
        let (focus, default) = shift_focus(self.state.focus, &self.focii, _action.clone());

        // Set focus, if the multifocus defaults, take no further action
        self.state.focus = focus;
        if let Some(_default) = default {
            return _default;
        }

        // Perform our state reduction
        self.state = reduce(self.state.clone(), _action.clone());

        match _action {
            Action::Patch(a,b,c) => _action,
            _ => Action::Noop
        }
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
