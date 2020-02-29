use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor, param_map};

use crate::common::{Screen, Window};
use crate::views::{Layer};
use crate::components::{popup, ivories};

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
        octave: match action {
            Action::Octave(true) => state.octave + 1,
            Action::Octave(false) => state.octave - 1,
            _ => state.octave
        },
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
        shift: state.shift,
        velocity: state.velocity
    }
}

impl Keyboard {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {
        let (_, params) = param_map(&mut doc);
        // Initialize State
        let initial_state: KeyboardState = KeyboardState {
            keys_active: vec![],
            octave: *params.get("octave").unwrap_or(&3.0) as usize,
            shift: *params.get("shift").unwrap_or(&0.0) as i8,
            velocity: *params.get("velocity").unwrap_or(&0.3)
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
        let win = Window {
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height
        };
        ivories::render(out, win, 5, &self.state.keys_active);

        write!(out, "{}OCT:{}", cursor::Goto(win.x, win.y), self.state.octave);
        write!(out, "{}SHIFT:{}", cursor::Goto(win.x, win.y+1), self.state.shift);
        write!(out, "{}VEL:{}", cursor::Goto(win.x, win.y+2), self.state.velocity);
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::Route => Action::ShowAnchors(vec![Anchor {
                index: 0,
                module_id: 0,
                name: "Keys".to_string(),
                input: false,
            }]),
            a @ Action::Up | 
            a @ Action::Down => a,
            _ => Action::Noop
        }
    }
    fn alpha(&self) -> bool { false }
    fn save(&self) -> Option<Element> { None }
}
