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
    focus: MultiFocus<ArpeggioState>,
}

#[derive(Clone, Debug)]
pub struct ArpeggioState {
    length: u32,
}

static VOID_RENDER: fn( &mut Screen, Window, ID, &ArpeggioState, bool) =
    |_, _, _, _, _| {};
static VOID_TRANSFORM: fn(Action, ID, &ArpeggioState) -> Action = 
    |_, _, _| Action::Noop;

fn reduce(state: ArpeggioState, action: Action) -> ArpeggioState {
    ArpeggioState {
        length: match action {
            Action::SetParam(ref key, val) if key == "length" => val as u32,
            _ => state.length
        }
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
            state: initial_state,
            focus: MultiFocus::<ArpeggioState> {
                r_id: (FocusType::Param, 0),
                r_t: |action, id, state| match action {
                    Action::Up => Action::SetParam("length".to_string(), (state.length + 1) as f32),
                    Action::Down => Action::SetParam("length".to_string(), (state.length - 1) as f32),
                    _ => Action::Noop,
                },
                r: |mut out, window, id, state, focus| {
                    bigtext::render(out, 
                        window.x + 5, 
                        window.y + 5, 
                        state.length.to_string());
                },
                y_id: VOID_ID.clone(),
                y_t: VOID_TRANSFORM,
                y: VOID_RENDER,
                p_id: VOID_ID.clone(),
                p_t: VOID_TRANSFORM,
                p: VOID_RENDER,
                b_id: VOID_ID.clone(),
                b_t: VOID_TRANSFORM,
                b: VOID_RENDER,
                g_id: VOID_ID.clone(),
                g_t: VOID_TRANSFORM,
                g: VOID_RENDER,
                w_id: VOID_ID.clone(),
                w: VOID_RENDER,
                active: None,
            }
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

        write!(out, "{}ARPEGGIATOR ALPHA", cursor::Goto(win.x, win.y));

        self.focus.render(out, win, &self.state, false, target);
    }
    fn dispatch(&mut self, action: Action) -> Action {

        let _action = self.focus.transform(action.clone(), &mut self.state);

        self.state = reduce(self.state.clone(), _action.clone());
        match _action {
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
        return Some(root)
    }
}
