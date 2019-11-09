use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color, Direction, write_fg, write_bg};
use crate::views::{Layer};
use crate::components::{piano, slider};

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

pub struct MultiFocus<T> {
    r: fn(RawTerminal<Stdout>, u16, u16, T) -> RawTerminal<Stdout>,
    g: fn(RawTerminal<Stdout>, u16, u16, T) -> RawTerminal<Stdout>,
    y: fn(RawTerminal<Stdout>, u16, u16, T) -> RawTerminal<Stdout>,
    p: fn(RawTerminal<Stdout>, u16, u16, T) -> RawTerminal<Stdout>,
    b: fn(RawTerminal<Stdout>, u16, u16, T) -> RawTerminal<Stdout>,
}

impl<T: Copy> MultiFocus<T> {
    pub fn render(&self, mut out: RawTerminal<Stdout>, x: u16, y: u16, state: T, active: bool) -> RawTerminal<Stdout> {
        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        out = (self.r)(out, x, y, state);

        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        out = (self.g)(out, x, y, state);

        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        out = (self.y)(out, x, y, state);
        
        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        out = (self.p)(out, x, y, state);

        if active { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Red); 
        }
        out = (self.b)(out, x, y, state);
        out
    }
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
        focus: (0,0),
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
            focii: vec![
                vec![
                    MultiFocus::<PianoState> {
                        r: |mut out, x, y, state| {
                            out = slider::render(out, 
                                x+8, 
                                y+5, 
                                "lo".to_string(), 
                                state.eq_low,
                                Direction::North,
                                Color::Transparent);
                            out
                        },
                        g: |mut out, x, y, state| {
                            out = slider::render(out, 
                                x+13, 
                                y+5, 
                                "mid".to_string(), 
                                state.eq_mid,
                                Direction::North,
                                Color::Transparent);
                            out
                        },
                        y: |mut out, x, y, state| {
                            out = slider::render(out, 
                                x+18, 
                                y+5, 
                                "hi".to_string(), 
                                state.eq_hi,
                                Direction::North,
                                Color::Transparent);
                            out
                        },
                        p: |mut out, x, y, state| {out},
                        b: |mut out, x, y, state| {out},
                    }
                ]
            ]
        }
    }
}

impl Layer for Piano {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        out = piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        for (i, row) in self.focii.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let isFocused = self.state.focus == (i,j);
                out = col.render(out, self.x, self.y, self.state.clone(), isFocused);
            }
        }

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();
        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        // We maintain a focus tuple
        // let (i,j) = self.focus;

        // We let our current focus transform the action
        // self.focii must be mutable
        // default = self.focii[i][j].reduce(action)

        self.state = reduce(self.state.clone(), action.clone());

        match action {
            // Action::Up | Action::Down | Action::Left | Action::Right => 
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
