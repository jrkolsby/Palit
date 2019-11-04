use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color};
use crate::views::{Layer};
use crate::components::{piano, slider};

pub struct Piano {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PianoState,
}

#[derive(Clone, Debug)]
pub struct PianoState {
    notes: Vec<Action>
}

fn reduce(state: PianoState, action: Action) -> PianoState {
    PianoState {
        notes: match action {
            Action::NoteOn(_,_) => { 
                println!("piano view noteon");
                let mut new_keys = state.notes.clone(); 
                new_keys.push(action);
                new_keys
            },
            Action::NoteOff(note) => {
                let mut new_keys = state.notes.clone();
                new_keys.retain(|a| match a {
                    Action::NoteOn(_note, _) => note == *_note,
                    _ => false,
                });
                new_keys
            }
            _ => state.notes.clone()
        }

    }
}

impl Piano {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: PianoState = PianoState {
            notes: vec![]
        };

        Piano {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Piano {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        out = piano::render(out, self.x, self.y, &self.state.notes);

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();
        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        println!("{:?}", action);
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
