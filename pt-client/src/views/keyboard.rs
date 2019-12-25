use std::io::{Write, Stdout};

use termion::{color};
use xmltree::Element;

use crate::common::{Screen, Action, Window};
use crate::views::{Layer};
use crate::components::{popup, ivories};
use crate::modules::param_map;

pub struct Keyboard {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: KeyboardState,
    history: Vec<KeyboardState>,
}

#[derive(Clone, Debug)]
pub struct KeyboardState {
    keys_active: Vec<Action>,
    octave: usize,
    shift: i8,
    velocity: f32,
}

fn reduce(state: KeyboardState, action: Action) -> KeyboardState {
    KeyboardState {
        keys_active: match action {
            a @ Action::NoteOn(_, _) => {
                let mut new_active = state.keys_active.clone();
                new_active.push(a);
                new_active
            },
            Action::NoteOff(k) => {
                let mut new_active = state.keys_active.clone();
                new_active.retain(|a| match a {
                    Action::NoteOn(_k,_) => *_k != k,
                    _ => false
                });
                new_active
            }
            _ => state.keys_active.clone()
        },
        octave: state.octave,
        shift: state.shift,
        velocity: state.velocity
    }
}

impl Keyboard {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Element) -> Self {
        let (_, params) = param_map(doc);
        // Initialize State
        let initial_state: KeyboardState = KeyboardState {
            keys_active: vec![],
            octave: *params.get("octave").unwrap_or(&4) as usize,
            shift: *params.get("shift").unwrap_or(&0) as i8,
            velocity: *params.get("velocity").unwrap_or(&500) as f32 / 1000.
        };

        Keyboard {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state
        }
    }
}

impl Layer for Keyboard {
    fn render(&self, out: &mut Screen, target: bool) {
        ivories::render(out, Window {
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height
        }, 3, &self.state.keys_active);
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            a @ Action::Up | a @ Action::Down => a,
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
