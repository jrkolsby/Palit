use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor, param_map};

use crate::views::{Layer};
use crate::components::{popup, ivories};
use crate::common::{Screen, Window, ID, VOID_ID, FocusType};
use crate::common::{MultiFocus, render_focii, shift_focus};

#[derive(Clone, Debug)]
struct FaustParam {
    label: String,
    init: f32,
    min: f32,
    max: f32,
    step: f32,
    value: f32,
}

pub struct Plugin {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PluginState,
    history: Vec<PluginState>,
    focii: Vec<Vec<MultiFocus<PluginState>>>,
}

#[derive(Clone, Debug)]
pub struct PluginState {
    params: Vec<FaustParam>,
    faust_anchors: Vec<(String, bool)>, // name, input
}

const PADDING: (u16, u16) = (5,5);

static VOID_RENDER: fn( &mut Screen, Window, ID, &PluginState, bool) =
    |_, _, _, _, _| {};
static VOID_TRANSFORM: fn(Action, ID, &PluginState) -> Action = 
    |_, _, _| Action::Noop;

fn generate_focii(params: &Vec<FaustParam>) -> Vec<Vec<MultiFocus<PluginState>>> {
    let void_focus = MultiFocus::<PluginState> {
        w_id: (FocusType::Void, 0),
        w: VOID_RENDER,
        r_id: (FocusType::Void, 0),
        r: VOID_RENDER,
        r_t: |_, _, _| Action::Noop,
        g_id: (FocusType::Void, 0), 
        g: VOID_RENDER,
        g_t: |_, _, _| Action::Noop,
        p_id:(FocusType::Void, 0), 
        p: VOID_RENDER,
        p_t: |_, _, _| Action::Noop,
        y_id:(FocusType::Void, 0), 
        y: VOID_RENDER,
        y_t: |_, _, _| Action::Noop,
        b_id: (FocusType::Void, 0), 
        b: VOID_RENDER,
        b_t: |_, _, _| Action::Noop,
        active: None,
    };

    let mut counter = 0;
    let mut focii = vec![];
    let mut focus_acc = void_focus.clone();

    for (i, param) in params.iter().enumerate() {
        let id = (FocusType::Param, i as u16);
        let transform: fn(Action, ID, &PluginState) -> Action = |a, id, state| {
            let param = &state.params[id.1 as usize];
            match a {
                Action::Up => Action::SetParam(param.label.clone(), param.value + param.step),
                Action::Down => Action::SetParam(param.label.clone(), param.value - param.step),
                _ => a
            }
        };
        let render: fn(&mut Screen, Window, ID, &PluginState, bool) = 
            |mut out, window, id, state, focus| {
                let param = &state.params[id.1 as usize];
                write!(out, "{}{}", cursor::Goto(
                    PADDING.0 + window.x, 
                    PADDING.1 + window.y + id.1 * 2,
                ), param.label).unwrap();
        };
        counter = match counter {
            0 => { focus_acc.r_id = id; focus_acc.r = render; focus_acc.r_t = transform; 1 },
            1 => { focus_acc.g_id = id; focus_acc.g = render; focus_acc.g_t = transform; 2 },
            2 => { focus_acc.p_id = id; focus_acc.p = render; focus_acc.p_t = transform; 3 },
            3 => { focus_acc.y_id = id; focus_acc.y = render; focus_acc.y_t = transform; 4 },
            _ => { focus_acc.b_id = id; focus_acc.b = render; focus_acc.b_t = transform; 0 },
        };
        if counter == 0 { 
            focii.push(vec![focus_acc]);
            focus_acc = void_focus.clone();
        }
    }
    if counter > 0 { focii.push(vec![focus_acc]); }
    focii
}

fn reduce(state: PluginState, action: Action) -> PluginState {
    PluginState {
        params: match action.clone() {
            Action::DeclareParam(label, init, min, max, step) => {
                let mut new_params = state.params.clone();
                new_params.push(FaustParam {
                    label,
                    init,
                    min,
                    max,
                    step,
                    value: init,
                });
                new_params
            },
            _ => state.params.clone()
        },
        faust_anchors: match action {
            Action::DeclareAnchors(ins, outs) => {
                let mut new_anchors = vec![];
                // Outputs first
                for i in 0..(outs / 2) {
                    new_anchors.push((format!("Audio Out {}", i), false))
                }
                for i in 0..(ins / 2) {
                    new_anchors.push((format!("Audio In {}", i), true))
                }
                new_anchors
            },
            _ => state.faust_anchors.clone()
        }
    }
}

impl Plugin {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {
        let (_, params) = param_map(&mut doc);
        // Initialize State
        let initial_state: PluginState = PluginState {
            params: vec![],
            faust_anchors: vec![],
        };

        Plugin {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state,
            focii: vec![],
        }
    }
}

impl Layer for Plugin {
    fn render(&self, out: &mut Screen, target: bool) {
        let win = Window {
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height
        };

        write!(out, "{}FAUST PLUGIN", cursor::Goto(win.x + PADDING.0, win.y + PADDING.1));
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::Route => Action::ShowAnchors({
                let mut anchors: Vec<Anchor> = self.state.faust_anchors
                    .iter().enumerate().map(|(i, anchor)| Anchor {
                        index: i as u16,
                        module_id: 0,
                        name: anchor.0.clone(),
                        input: anchor.1.clone(),
                    }).collect();
                if self.state.params.iter().find(|&p| p.label == "freq").is_some() &&
                    self.state.params.iter().find(|&p| p.label == "gain").is_some() {
                        anchors.push(Anchor {
                            index: anchors.len() as u16,
                            module_id: 0,
                            name: format!("MIDI In"),
                            input: true,
                        });
                    }
                anchors
                
            }),
            a @ Action::Left |
            a @ Action::Up | 
            a @ Action::Down => a,
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
