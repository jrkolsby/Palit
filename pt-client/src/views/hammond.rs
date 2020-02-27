use std::io::{Write, Stdout};
use xmltree::Element;
use libcommon::{Action, Anchor, Param, param_map, param_add};

use crate::common::{MultiFocus, shift_focus, render_focii, focus_dispatch};
use crate::common::{Screen, Direction, FocusType, Window};
use crate::views::{Layer};
use crate::components::{piano, slider, button};

pub struct Hammond {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: HammondState,
    focii: Vec<Vec<MultiFocus<HammondState>>>,
}

#[derive(Clone, Debug)]
pub struct HammondState {
    focus: (usize, usize),
    notes: Vec<Action>,
    eq: [Param; 9],
}

fn reduce(state: HammondState, action: Action) -> HammondState {
    HammondState {
        notes: match action {
            Action::NoteOn(_,_) => { 
                let mut new_keys = state.notes.clone(); 
                new_keys.push(action.clone());
                new_keys
            },
            Action::NoteOff(note) => {
                let mut new_keys = state.notes.clone();
                new_keys.retain(|a| match a {
                    Action::NoteOn(_note, _) => note != *_note,
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
const EQ_STEP: Param = 0.001;
const EQ_FACTOR: Param = 200.0;

impl Hammond {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {

        let (_, params) = param_map(&mut doc);

        let initial_eq = [
            *params.get("16").unwrap_or(&0.01),
            *params.get("5.3").unwrap_or(&0.01),
            *params.get("8").unwrap_or(&0.01),
            *params.get("4").unwrap_or(&0.01),
            *params.get("2.6").unwrap_or(&0.01),
            *params.get("2").unwrap_or(&0.01),
            *params.get("1.6").unwrap_or(&0.01),
            *params.get("1.3").unwrap_or(&0.01),
            *params.get("1").unwrap_or(&0.01),
        ];

        // Initialize State
        let initial_state: HammondState = HammondState {
            focus: (0,0),
            notes: vec![],
            eq: initial_eq,
        };

        Hammond {
            x: x + if SIZE.0 > width { 0 } else { (width / 2) - (SIZE.0 / 2) } ,
            y: y + if SIZE.1 > height { 0 } else { height - SIZE.1 },
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<HammondState> {
                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+5, window.y+5, "16'".to_string(), 
                            (state.eq[0] * EQ_FACTOR) as i16, Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("16".to_string(), 
                                                         state.eq[0] + EQ_STEP) },
                        Action::Down => { Action::SetParam("16".to_string(), 
                                                         state.eq[0] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+10, window.y+5, "5⅓'".to_string(), 
                            (state.eq[1] * EQ_FACTOR) as i16, Direction::North)
                    },
                    b_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("5.3".to_string(), 
                                                         state.eq[1] + EQ_STEP) },
                        Action::Down => { Action::SetParam("5.3".to_string(), 
                                                         state.eq[1] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+15, window.y+5, "8'".to_string(), 
                            (state.eq[2] * EQ_FACTOR) as i16, Direction::North)
                    },
                    p_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("8".to_string(), 
                                                         state.eq[2] + EQ_STEP) },
                        Action::Down => { Action::SetParam("8".to_string(), 
                                                         state.eq[2] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+20, window.y+5, "4'".to_string(), 
                            (state.eq[3] * EQ_FACTOR) as i16, Direction::North)
                    },
                    g_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("4".to_string(), 
                                                         state.eq[3] + EQ_STEP) },
                        Action::Down => { Action::SetParam("4".to_string(), 
                                                         state.eq[3] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    g_id: (FocusType::Button, 0),
                    w: |mut out, window, id, state, focus| {},
                    w_id: (FocusType::Void, 0),
                    r: |mut out, window, id, state, focus| {},
                    r_t: |action, id, state| Action::Noop,
                    r_id: (FocusType::Void, 0),
                    active: None,
                },
                MultiFocus::<HammondState> {
                    w: |mut out, window, id, state, focus| {},
                    w_id: (FocusType::Void, 0),

                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+25, window.y+5, "2⅔'".to_string(), 
                            (state.eq[4] * EQ_FACTOR) as i16, Direction::North)
                    },
                    y_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("2.6".to_string(), 
                                                         state.eq[4] + EQ_STEP) },
                        Action::Down => { Action::SetParam("2.6".to_string(), 
                                                         state.eq[4] - EQ_STEP) },
                        _ => Action::Noop
                    }, 
                    y_id: (FocusType::Button, 0),

                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+30, window.y+5, "2'".to_string(), 
                            (state.eq[5] * EQ_FACTOR) as i16, Direction::North)
                    },
                    b_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("2".to_string(), 
                                                         state.eq[5] + EQ_STEP) },
                        Action::Down => { Action::SetParam("2".to_string(), 
                                                         state.eq[5] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+35, window.y+5, "1⅗'".to_string(), 
                            (state.eq[6] * EQ_FACTOR) as i16, Direction::North)
                    },
                    p_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("1.6".to_string(), 
                                                         state.eq[6] + EQ_STEP) },
                        Action::Down => { Action::SetParam("1.6".to_string(), 
                                                         state.eq[6] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+40, window.y+5, "1⅓'".to_string(), 
                            (state.eq[7] * EQ_FACTOR) as i16, Direction::North)
                    },
                    g_t: |action, id, state| match action {
                        Action::Up => { Action::SetParam("1.3".to_string(), 
                                                         state.eq[7] + EQ_STEP) },
                        Action::Down => { Action::SetParam("1.3".to_string(), 
                                                         state.eq[7] - EQ_STEP) },
                        _ => Action::Noop
                    },
                    g_id: (FocusType::Button, 0),
                    r: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+45, window.y+5, "1'".to_string(), 
                            (state.eq[8] * EQ_FACTOR) as i16, Direction::North)
                    },
                    r_t: |action, id, state| match action { 
                        Action::Up => { Action::SetParam("1".to_string(), 
                                                         state.eq[8] + EQ_STEP) },
                        Action::Down => { Action::SetParam("1".to_string(), 
                                                         state.eq[8] - EQ_STEP) },
                        _ => Action::Noop
                    }, 
                    r_id: (FocusType::Button, 0),
                    active: None,
                },
            ]]
        }
    }
}

impl Layer for Hammond {
    fn render(&self, out: &mut Screen, target: bool) {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        render_focii(out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, false, !target);
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
                a @ Action::Left |
                a @ Action::Up | 
                a @ Action::Down |
                a @ Action::SetParam(_,_) => a,
                _ => { Action::Noop }
            }
        } else { Action::Noop }
    }
    fn alpha(&self) -> bool { false }
    fn save(&self) -> Option<Element> { 
        let mut root = Element::new("hammond");
        param_add(&mut root, self.state.eq[0], "16".to_string());
        param_add(&mut root, self.state.eq[1], "5.3".to_string());
        param_add(&mut root, self.state.eq[2], "8".to_string());
        param_add(&mut root, self.state.eq[3], "4".to_string());
        param_add(&mut root, self.state.eq[4], "2.6".to_string());
        param_add(&mut root, self.state.eq[5], "2".to_string());
        param_add(&mut root, self.state.eq[6], "1.6".to_string());
        param_add(&mut root, self.state.eq[7], "1.3".to_string());
        param_add(&mut root, self.state.eq[8], "1".to_string());
        Some(root)
    }
}
