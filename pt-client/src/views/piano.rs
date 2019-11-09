use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Direction, MultiFocus};
use crate::views::{Layer};
use crate::components::{piano, slider, button};

pub struct Piano {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PianoState,
    focii: Vec<Vec<MultiFocus<PianoState>>>,
}

#[derive(Clone, Debug)]
pub struct PianoState {
    focus: (usize, usize),
    notes: Vec<Action>,
    eq_low: i16,
    eq_mid: i16,
    eq_hi: i16,
}

fn reduce(state: PianoState, action: Action) -> PianoState {
    PianoState {
        notes: match action {
            Action::NoteOn(_,_) => { 
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
            },
            _ => state.notes.clone()
        },
        eq_low: state.eq_low,
        eq_mid: state.eq_mid,
        eq_hi: state.eq_hi,
        focus: state.focus,
    }
}

impl Piano {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: PianoState = PianoState {
            focus: (0,0),
            notes: vec![],
            eq_low: 8,
            eq_mid: 8,
            eq_hi: 8,
        };

        Piano {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<PianoState> {
                    r: |mut out, x, y, state| {
                        button::render(out, x+2, y+10, 20, "Record")
                    },
                    r_t: |action, state| { action },
                    g: |mut out, x, y, state| {
                        slider::render(out, x+8, y+5, "20Hz".to_string(), 
                            state.eq_mid, Direction::North)
                    },
                    g_t: |action, state| match action {
                        Action::Up => { state.eq_mid += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_mid -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    y: |mut out, x, y, state| {
                        slider::render(out, x+14, y+5, "80Hz".to_string(), 
                            state.eq_hi, Direction::North)
                    },
                    y_t: |action, state| match action {
                        Action::Up => { state.eq_hi += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_hi -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    p: |mut out, x, y, state| {
                        slider::render(out, x+20, y+5, "120Hz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    p_t: |action, state| match action { 
                        Action::Up => { state.eq_low += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_low -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    b: |mut out, x, y, state| {
                        slider::render(out, x+26, y+5, "400Hz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    b_t: |action, state| match action { 
                        Action::Up => { state.eq_low += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_low -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    active: None,
                },
                MultiFocus::<PianoState> {
                    r: |mut out, x, y, state| {
                        button::render(out, x+32, y+10, 10, "Play")
                    },
                    r_t: |action, state| action,
                    g: |mut out, x, y, state| {
                        slider::render(out, x+32, y+5, "6KHz".to_string(), 
                            state.eq_mid, Direction::North)
                    },
                    g_t: |action, state| action,
                    y: |mut out, x, y, state| {
                        slider::render(out, x+38, y+5, "12KHz".to_string(), 
                            state.eq_hi, Direction::North)
                    },
                    y_t: |action, state| action,
                    p: |mut out, x, y, state| {
                        slider::render(out, x+44, y+5, "14KHz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    p_t: |action, state| action,
                    b: |mut out, x, y, state| {
                        slider::render(out, x+50, y+5, "20KHz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    b_t: |action, state| action,
                    active: None,
                },
            ]]
        }
    }
}

impl Layer for Piano {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        out = piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        if let Some(_) = self.focii[self.state.focus.1][self.state.focus.0].active {
            out = self.focii[self.state.focus.1][self.state.focus.0].render(out, 
                self.x, self.y, &self.state.clone(), true);
        } else {
            for (j, col) in self.focii.iter().enumerate() {
                for (i, focus) in col.iter().enumerate() {
                    let isFocused = self.state.focus == (i,j);
                    out = focus.render(out, self.x, self.y, &self.state.clone(), isFocused);
                }
            }
        }

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();
        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        let focus_row = &self.focii[self.state.focus.1];
        let focus_i = &focus_row[self.state.focus.0];

        let mut default: Option<Action> = None;
        if focus_i.active.is_none() {
            // self.focii must be mutable
            // default = self.focii[i][j].reduce(action)
            self.state.focus = match action {
                Action::Left => if self.state.focus.0 > 0 {
                        (self.state.focus.0-1, self.state.focus.1)
                    } else { default = Some(Action::Left); self.state.focus },
                Action::Right => if self.state.focus.0 < (focus_row.len()-1) {
                        (self.state.focus.0+1, self.state.focus.1)
                    } else { default = Some(Action::Right); self.state.focus },
                Action::Up => if self.state.focus.1 > 0 {
                        (self.state.focus.0, self.state.focus.1-1)
                    } else { default = Some(Action::Up); self.state.focus }
                Action::Down => if self.state.focus.1 < (self.focii.len()-1) {
                        (self.state.focus.0, self.state.focus.1+1)
                    } else { default = Some(Action::Down); self.state.focus },
                _ => self.state.focus
            };
        }

        // If the user tried to exceed the focus bounds, pass default back up 
        // to the caller and take no further action
        if let Some(_default) = default {
            return _default;
        }

        // Otherwise, let the new focus transform the action
        let _action = self.focii[self.state.focus.1][self.state.focus.0].transform(action, &mut self.state);

        // Perform our state reduction
        self.state = reduce(self.state.clone(), _action.clone());

        // This is not comprehensive
        return match _action {
            Action::SetParam(_,_) => _action,
            _ => Action::Noop
        };
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
