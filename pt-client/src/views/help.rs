use std::io::{Write, Stdout};

use termion::{color};
use termion::raw::{RawTerminal};

use crate::common::Action;
use crate::views::{Layer};
use crate::components::{popup, keyboard};

pub struct Help {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: HelpState,
    history: Vec<HelpState>,
}

#[derive(Clone, Debug)]
pub struct HelpState {
    title: String,
    active: Vec<Action>
}

fn reduce(state: HelpState, action: Action) -> HelpState {
    HelpState {
        title: state.title.clone(),
        active: match action {
            Action::NoteOn(_, _) => {
                let mut new_active = state.active.clone();
                new_active.push(action);
                new_active
            }
            _ => state.active.clone()
        },
    }
}

impl Help {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // Initialize State
        let initial_state: HelpState = HelpState {
            title: "KEYBOARD".to_string(),
            active: vec![]
        };

        Help {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state
        }
    }
}

impl Layer for Help {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

	    out = popup::render(out, self.x, self.y, self.width, self.height, &self.state.title);
        out = keyboard::render(out, &self.state.active, self.x+5, self.y+5);

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::SelectR => Action::Back,
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
    fn shift(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
