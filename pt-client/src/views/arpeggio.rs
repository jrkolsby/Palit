use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor, param_map};

use crate::common::{Screen, Window};
use crate::views::{Layer};
use crate::components::{popup, ivories};

pub struct Arpeggio {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: ArpeggioState,
    history: Vec<ArpeggioState>,
}

#[derive(Clone, Debug)]
pub struct ArpeggioState {
    length: u32,
}

fn reduce(state: ArpeggioState, action: Action) -> ArpeggioState {
    ArpeggioState {
        length: state.length
    }
}

impl Arpeggio {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {
        let (_, params) = param_map(&mut doc);
        // Initialize State
        let initial_state: ArpeggioState = ArpeggioState {
            length: *params.get("length").unwrap_or(&4.0) as u32
        };

        Arpeggio {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state
        }
    }
}

impl Layer for Arpeggio {
    fn render(&self, out: &mut Screen, target: bool) {
        let win = Window {
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height
        };

        write!(out, "{}ARPEGGIO {}", cursor::Goto(win.x, win.y), self.state.length);
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::Route => Action::ShowAnchors(vec![Anchor {
                index: 0,
                module_id: 0,
                name: "MIDI Out".to_string(),
                input: false,
            },
            Anchor {
                index: 1,
                module_id: 0,
                name: "MIDI In".to_string(),
                input: true,
            }]),
            a @ Action::Left |
            a @ Action::Up | 
            a @ Action::Down => a,
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
        false
    }
}
