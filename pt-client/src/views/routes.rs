use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};
use xmltree::Element;

use crate::common::{MultiFocus, shift_focus, render_focii, FocusType};
use crate::common::{Action, Window, Anchor};
use crate::views::{Layer};
use crate::components::{button};

#[derive(Clone, Debug)]
struct Route {
    id: u16,
    patch: Vec<Anchor>,
}

#[derive(Clone, Debug)]
pub struct RoutesState {
    routes: Vec<Route>,
    anchors: Vec<Anchor>,
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

fn generate_focii(
    routes: &Vec<Route>, 
    anchors: &Vec<Anchor>
) -> Vec<Vec<MultiFocus::<RoutesState>>> {
    vec![vec![]]
}

fn reduce(state: RoutesState, action: Action) -> RoutesState {
    RoutesState {
        routes: match action {
            Action::AddRoute(a) => {
                let mut new_routes = state.routes.clone();
                new_routes.push(Route {
                    id: a,
                    patch: vec![]
                });
                new_routes
            },
            _ => state.routes.clone()
        },
        anchors: match action {
            Action::ShowAnchors(a) => a,
            _ => state.anchors.clone()
        },
        focus: state.focus,
    }
}

impl Routes {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Option<Element>) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: RoutesState = RoutesState {
            routes: vec![],
            anchors: vec![],
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
                    w: |mut out, window, id, state, focus| out,
                    w_id: (FocusType::Void, 0),
                    r: |mut out, window, id, state, focus| out,
                    r_t: |action, id, state| action,
                    r_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| out,
                    g_t: |action, id, state| action,
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, id, state, focus| out,
                    y_t: |action, id, state| action,
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| out,
                    p_t: |action, id, state| action,
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| out,
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

        println!("{} ANCHORS", self.state.anchors.len());

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
            Action::ShowAnchors(_) => Action::CountRoutes(self.state.routes.len() as u16),
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
    fn shift(&mut self, x: u16, y: u16) {}
}
