use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color};
use crate::views::{Layer};
use crate::components::{piano, slider};

pub struct Routes {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: RoutesState,
}

#[derive(Clone, Debug)]
pub struct RoutesState {
    num_routes: u16,
}

fn reduce(state: RoutesState, action: Action) -> RoutesState {
    RoutesState {
        num_routes: state.num_routes
    }
}

impl Routes {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: RoutesState = RoutesState {
            num_routes: 1,
        };

        Routes {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Routes {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        for i in 0..self.state.num_routes {
            for j in 1..self.height {
                write!(out, "{}{}{}│", cursor::Goto(i,j), color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();
            }
        }

        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());

        match action {
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
