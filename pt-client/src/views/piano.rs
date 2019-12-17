use std::io::{Write, Stdout};
use termion::raw::{RawTerminal};
use xmltree::Element;

use crate::common::{MultiFocus, shift_focus, render_focii, focus_dispatch};
use crate::common::{Action, Direction, FocusType, Window, Anchor};
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
    eq: [i16; 8],
}

fn reduce(state: PianoState, action: Action) -> PianoState {
    PianoState {
        notes: match action {
            Action::NoteOn(_,_) => { 
                let mut new_keys = state.notes.clone(); 
                new_keys.push(action.clone());
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
        eq: match action {
            Action::SetParam(key, val) => {
                let mut new_eq = state.eq.clone();
                match key.as_ref() {
                    "20Hz" => new_eq[0] = val,
                    "80Hz" => new_eq[1] = val,
                    "120Hz" => new_eq[2] = val,
                    "140Hz" => new_eq[3] = val,
                    "400Hz" => new_eq[4] = val,
                    "6KHz" => new_eq[5] = val,
                    "12KHz" => new_eq[6] = val,
                    "14KHz" => new_eq[7] = val,
                    _ => {},
                };
                new_eq
            }
            _ => state.eq.clone()
        },
        focus: state.focus,
    }
}

impl Piano {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Element) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: PianoState = PianoState {
            focus: (0,0),
            notes: vec![],
            eq: [0; 8],
        };

        Piano {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<PianoState> {
                    r: |mut out, window, id, state, focus| {
                        button::render(out, window.x+2, window.y+16, 20, "Record")
                    },
                    r_t: |action, id, state| Action::Record,
                    r_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+8, window.y+5, "20Hz".to_string(), 
                            state.eq[0], Direction::North)
                    },
                    g_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("20Hz".to_string(), 
                                                         state.eq[0]+1) },
                        Action::Down => { Action::SetParam("20Hz".to_string(), 
                                                         state.eq[0]-1) },
                        _ => Action::Noop
                    },
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+14, window.y+5, "80Hz".to_string(), 
                            state.eq[1], Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("80Hz".to_string(), 
                                                         state.eq[1]+1) },
                        Action::Down => { Action::SetParam("80Hz".to_string(), 
                                                         state.eq[1]-1) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+20, window.y+5, "120Hz".to_string(), 
                            state.eq[2], Direction::North)
                    },
                    p_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("120Hz".to_string(), 
                                                         state.eq[2]+1) },
                        Action::Down => { Action::SetParam("120Hz".to_string(), 
                                                         state.eq[2]-1) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+26, window.y+5, "400Hz".to_string(), 
                            state.eq[3], Direction::North)
                    },
                    b_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("400Hz".to_string(), 
                                                         state.eq[3]+1) },
                        Action::Down => { Action::SetParam("400Hz".to_string(), 
                                                         state.eq[3]-1) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    w: |mut out, window, id, state, focus| out,
                    w_id: (FocusType::Void, 0),
                    active: None,
                },
                MultiFocus::<PianoState> {
                    w: |mut out, window, id, state, focus| out,
                    w_id: (FocusType::Void, 0),
                    r: |mut out, window, id, state, focus| {
                        button::render(out, window.x+32, window.y+16, 10, "Play")
                    },
                    r_t: |action, id, state| Action::Play,
                    r_id: (FocusType::Button, 0),

                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+32, window.y+5, "6KHz".to_string(), 
                            state.eq[4], Direction::North)
                    },
                    g_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("6KHz".to_string(), 
                                                         state.eq[4]+1) },
                        Action::Down => { Action::SetParam("6KHz".to_string(), 
                                                         state.eq[4]-1) },
                        _ => Action::Noop
                    }, 
                    g_id: (FocusType::Button, 0),

                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+38, window.y+5, "12KHz".to_string(), 
                            state.eq[5], Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("12KHz".to_string(), 
                                                         state.eq[5]+1) },
                        Action::Down => { Action::SetParam("12KHz".to_string(), 
                                                         state.eq[5]-1) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+44, window.y+5, "14KHz".to_string(), 
                            state.eq[6], Direction::North)
                    },
                    p_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("14KHz".to_string(), 
                                                         state.eq[6]+1) },
                        Action::Down => { Action::SetParam("14KHz".to_string(), 
                                                         state.eq[6]-1) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+50, window.y+5, "20KHz".to_string(), 
                            state.eq[7], Direction::North)
                    },
                    b_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("20KHz".to_string(), 
                                                         state.eq[7]+1) },
                        Action::Down => { Action::SetParam("20KHz".to_string(), 
                                                         state.eq[7]-1) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    active: None,
                },
            ]]
        }
    }
}

impl Layer for Piano {
    fn render(&self, mut out: RawTerminal<Stdout>, target: bool) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        out = render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, !target);

        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Intercept arrow actions to change focus
        let (focus, default) = focus_dispatch(self.state.focus, 
                                              &mut self.focii, 
                                              &self.state, 
                                              action.clone());
        self.state.focus = focus;

        // Set focus, if the multifocus defaults, take no further action
        if let Some(_default) = default {
            self.state = reduce(self.state.clone(), _default.clone());
            match _default {
                Action::Route => {
                    Action::ShowAnchors(vec![
                        Anchor {
                            id: 0, 
                            module_id: 0,
                            name: "Out".to_string(),
                            x: 15,
                            y: 9,
                            input: false,
                        }, Anchor {
                            id: 1, 
                            module_id: 0,
                            name: "Keys".to_string(),
                            x: 10,
                            y: 8,
                            input: true,
                        }
                    ])
                },
                a @ Action::Up | a @ Action::Down => a,
                _ => { Action::Noop }
            }
        } else { Action::Noop }
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
    fn shift(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
