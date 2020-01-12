use std::io::{Write, Stdout};
use xmltree::Element;

use crate::common::{MultiFocus, shift_focus, render_focii, focus_dispatch};
use crate::common::{Screen, Action, Direction, FocusType, Window, Anchor};
use crate::modules::param_map;
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
    eq: [i16; 9],
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
                    "16" => new_eq[0] = val,
                    "5.3" => new_eq[1] = val,
                    "8" => new_eq[2] = val,
                    "4" => new_eq[3] = val,
                    "2.6" => new_eq[4] = val,
                    "2" => new_eq[5] = val,
                    "1.6" => new_eq[6] = val,
                    "1.3" => new_eq[7] = val,
                    "1" => new_eq[8] = val,
                    _ => {},
                };
                new_eq
            }
            _ => state.eq.clone()
        },
        focus: state.focus,
    }
}

const SIZE: (u16, u16) = (70, 30);

impl Piano {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Element) -> Self {

        let (_, params) = param_map(doc);

        let initial_eq = [
            *params.get("16").unwrap(),
            *params.get("5.3").unwrap(),
            *params.get("8").unwrap(),
            *params.get("4").unwrap(),
            *params.get("2.6").unwrap(),
            *params.get("2").unwrap(),
            *params.get("1.6").unwrap(),
            *params.get("1.3").unwrap(),
            *params.get("1").unwrap(),
        ];

        // Initialize State
        let initial_state: PianoState = PianoState {
            focus: (0,0),
            notes: vec![],
            eq: initial_eq,
        };

        Piano {
            x: x + (width / 2) - (SIZE.0 / 2),
            y: y + height - SIZE.1,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<PianoState> {
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+5, window.y+5, "16'".to_string(), 
                            state.eq[0], Direction::North)
                    },
                    g_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("16".to_string(), 
                                                         state.eq[0]+1) },
                        Action::Down => { Action::SetParam("16".to_string(), 
                                                         state.eq[0]-1) },
                        _ => Action::Noop
                    },
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+10, window.y+5, "5⅓'".to_string(), 
                            state.eq[1], Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("5.3".to_string(), 
                                                         state.eq[1]+1) },
                        Action::Down => { Action::SetParam("5.3".to_string(), 
                                                         state.eq[1]-1) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+15, window.y+5, "8'".to_string(), 
                            state.eq[2], Direction::North)
                    },
                    p_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("8".to_string(), 
                                                         state.eq[2]+1) },
                        Action::Down => { Action::SetParam("8".to_string(), 
                                                         state.eq[2]-1) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+20, window.y+5, "4'".to_string(), 
                            state.eq[3], Direction::North)
                    },
                    b_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("4".to_string(), 
                                                         state.eq[3]+1) },
                        Action::Down => { Action::SetParam("4".to_string(), 
                                                         state.eq[3]-1) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    w: |mut out, window, id, state, focus| {},
                    w_id: (FocusType::Void, 0),
                    r: |mut out, window, id, state, focus| {},
                    r_t: |action, id, state| Action::Noop,
                    r_id: (FocusType::Void, 0),
                    active: None,
                },
                MultiFocus::<PianoState> {
                    w: |mut out, window, id, state, focus| {},
                    w_id: (FocusType::Void, 0),

                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+25, window.y+5, "2⅔'".to_string(), 
                            state.eq[4], Direction::North)
                    },
                    g_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("2.6".to_string(), 
                                                         state.eq[4]+1) },
                        Action::Down => { Action::SetParam("2.6".to_string(), 
                                                         state.eq[4]-1) },
                        _ => Action::Noop
                    }, 
                    g_id: (FocusType::Button, 0),

                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+30, window.y+5, "2'".to_string(), 
                            state.eq[5], Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("2".to_string(), 
                                                         state.eq[5]+1) },
                        Action::Down => { Action::SetParam("2".to_string(), 
                                                         state.eq[5]-1) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+35, window.y+5, "1⅗'".to_string(), 
                            state.eq[6], Direction::North)
                    },
                    p_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("1.6".to_string(), 
                                                         state.eq[6]+1) },
                        Action::Down => { Action::SetParam("1.6".to_string(), 
                                                         state.eq[6]-1) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+40, window.y+5, "1⅓'".to_string(), 
                            state.eq[7], Direction::North)
                    },
                    b_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("1.3".to_string(), 
                                                         state.eq[7]+1) },
                        Action::Down => { Action::SetParam("1.3".to_string(), 
                                                         state.eq[7]-1) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    r: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+45, window.y+5, "1'".to_string(), 
                            state.eq[8], Direction::North)
                    },
                    r_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("1".to_string(), 
                                                         state.eq[8]+1) },
                        Action::Down => { Action::SetParam("1".to_string(), 
                                                         state.eq[8]-1) },
                        _ => Action::Noop
                    }, 
                    r_id: (FocusType::Button, 0),
                    active: None,
                },
            ]]
        }
    }
}

impl Layer for Piano {
    fn render(&self, out: &mut Screen, target: bool) {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        render_focii(out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, !target);
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
                    Action::ShowAnchors( vec![ 
                        Anchor {
                            index: 0, 
                            module_id: 0,
                            name: "Speaker".to_string(),
                            input: false,
                        },
                        Anchor {
                            index: 1, 
                            module_id: 0,
                            name: "MIDI In".to_string(),
                            input: true,
                        }])
                },
                a @ Action::Up | a @ Action::Down => a,
                a @ Action::SetParam(_,_) => a,
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
}
