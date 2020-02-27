use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor, param_map, param_add};

use crate::common::{MultiFocus, FocusType, ID, VOID_ID};
use crate::common::{render_focii, focus_dispatch};
use crate::common::{Screen, Window};
use crate::views::{Layer};
use crate::components::{popup, ivories, bigtext};

pub struct Arpeggio {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: ArpeggioState,
    history: Vec<ArpeggioState>,
    focii: Vec<Vec<MultiFocus<ArpeggioState>>>,
}

#[derive(Clone, Debug)]
pub struct ArpeggioState {
    length: f32,
    pattern: usize,
}

static VOID_RENDER: fn( &mut Screen, Window, ID, &ArpeggioState, bool) =
    |_, _, _, _, _| {};
static VOID_TRANSFORM: fn(Action, ID, &ArpeggioState) -> Action = 
    |_, _, _| Action::Noop;

fn reduce(state: ArpeggioState, action: Action) -> ArpeggioState {
    ArpeggioState {
        length: match action {
            Action::SetParam(ref key, val) if key == "length" => val,
            _ => state.length
        },
        pattern: match action {
            Action::SetParam(ref key, val) if key == "pattern" => val as usize,
            _ => state.pattern
        },
    }
}

const TITLE: &str = "ARPEGGIOHYEAH";
const NUM_PATTERNS: usize = 3;

impl Arpeggio {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {
        let (_, params) = param_map(&mut doc);
        // Initialize State
        let initial_state: ArpeggioState = ArpeggioState {
            length: *params.get("length").unwrap_or(&4.0),
            pattern: *params.get("pattern").unwrap_or(&0.0) as usize,
        };

        Arpeggio {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state,
            focii: vec![vec![MultiFocus::<ArpeggioState> {
                r_id: (FocusType::Param, 0),
                r_t: |action, id, state| match action {
                    Action::Up => Action::SetParam(
                        "length".to_string(), 
                        match state.length {
                            x if x >= 1.0 => x + 1.0,
                            x => x * 2.0,
                        }),
                    Action::Down => Action::SetParam(
                        "length".to_string(), 
                        match state.length {
                            x if x <= 1.0 => x / 2.0,
                            x => x - 1.0,
                        }),
                    _ => Action::Noop,
                },
                r: |mut out, window, id, state, focus| {
                    let out_size = 3 * state.length.to_string().len() as u16;
                    bigtext::render(out, 
                        window.x + (window.w / 2) - (out_size / 2), 
                        window.y + 5, 
                        state.length.to_string());
                },
                y_id: VOID_ID.clone(),
                y_t: VOID_TRANSFORM,
                y: VOID_RENDER,
                p_id: VOID_ID.clone(),
                p_t: VOID_TRANSFORM,
                p: VOID_RENDER,
                g_id: VOID_ID.clone(),
                g_t: VOID_TRANSFORM,
                g: VOID_RENDER,
                b_id: (FocusType::Param, 0),
                b_t: |action, id, state| match action {
                    Action::SelectB => Action::SetParam(
                        "pattern".to_string(), 
                        ((state.pattern + 1) % NUM_PATTERNS) as f32
                    ),
                    _ => Action::Noop,
                },
                b: |mut out, window, id, state, focus| {
                    bigtext::render(out, 
                        window.x + 5, 
                        window.y + 10, 
                        match state.pattern {
                            0 => "UP DN".to_string(),
                            1 => "DN UP".to_string(),
                            2 => "SEQ".to_string(),
                            _ => "ERR".to_string(),
                        });
                },
                w_id: (FocusType::Button, 0),
                w: |mut out, window, id, state, focus| {
                    write!(out, "{}{}", cursor::Goto(
                        window.x + (window.w / 2) - (TITLE.len() as u16 / 2), 
                        window.y + 1, 
                    ), TITLE);
                },
                active: None,
            }]]
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

        render_focii(out, win, (0,0), &self.focii, &self.state, false, !target);
    }
    fn dispatch(&mut self, action: Action) -> Action {

        let (_, _action) = focus_dispatch((0,0),
                                          &mut self.focii, 
                                          &self.state, 
                                          action.clone());

        self.state = reduce(self.state.clone(), _action.clone().unwrap());
        match _action.unwrap() {
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
            a @ Action::SetParam(_,_) |
            a @ Action::Left |
            a @ Action::Up | 
            a @ Action::Down => a,
            _ => Action::Noop
        }
    }
    fn alpha(&self) -> bool { false }
    fn save(&self) -> Option<Element> {
        let mut root = Element::new("arpeggio");
        param_add(&mut root, self.state.length, "length".to_string());
        param_add(&mut root, self.state.pattern, "pattern".to_string());
        return Some(root)
    }
}
