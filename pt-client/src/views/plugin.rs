use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor, Param, param_map};

use crate::views::{Layer};
use crate::components::{popup, ivories, slider};
use crate::common::{Screen, Window, ID, VOID_ID, FocusType, Direction};
use crate::common::{MultiFocus, render_focii, shift_focus};

#[derive(Clone, Debug)]
struct FaustParam {
    label: String,
    init: Param,
    min: Param,
    max: Param,
    step: Param,
    value: Param,
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
    focus: (usize, usize),
}

const PADDING: (u16, u16) = (5,5);
const PARAM_WIDTH: f32 = 40.0;

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
                Action::Up | 
                Action::Right => {
                    let mut target = param.value + param.step;
                    if target > param.max { target = param.max };
                    Action::SetParam(param.label.clone(), target)
                },
                Action::Down | 
                Action::Left => {
                    let mut target = param.value - param.step;
                    if target < param.min { target = param.min };
                    Action::SetParam(param.label.clone(), target)
                },
                _ => a
            }
        };
        let render: fn(&mut Screen, Window, ID, &PluginState, bool) = 
            |mut out, window, id, state, focus| {
                let param = &state.params[id.1 as usize];
                let factor = PARAM_WIDTH / param.max;
                slider::render(out,
                    PADDING.0 + window.x, 
                    PADDING.1 + window.y + id.1 * 3,
                    format!("{} ({})", param.label, param.value),
                    (param.value * factor) as i16, 
                    Direction::East);

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
            Action::SetParam(key, val) => {
                let mut new_params = state.params.clone();
                if let Some(mut param) = new_params.iter_mut().find(|p| p.label == key) {
                    param.value = val;
                }
                new_params
            },
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
        },
        focus: state.focus,
    }
}

impl Plugin {
    pub fn new(x: u16, y: u16, width: u16, height: u16, mut doc: Element) -> Self {
        let (_, params) = param_map(&mut doc);
        // Initialize State
        let initial_state: PluginState = PluginState {
            params: vec![],
            faust_anchors: vec![],
            focus: (0, 0),
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

        render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, false, !target);

        write!(out, "{}FAUST PLUGIN", cursor::Goto(win.x + PADDING.0, win.y + 2));
    }
    fn dispatch(&mut self, action: Action) -> Action {
        // Let the focus transform the action 
        let _action = {
            if let Some(multi_focus_row) = &mut self.focii.get_mut(self.state.focus.1) {
                if let Some(multi_focus) = &mut multi_focus_row.get_mut(self.state.focus.0) {
                    multi_focus.transform(action.clone(), &mut self.state)
                } else {
                    action
                }
            } else {
                action
            }
        };

        self.state = reduce(self.state.clone(), _action.clone());

        let (focus, default) = match _action {
            Action::DeclareParam(_,_,_,_,_) => {
                self.focii = generate_focii(&self.state.params);
                (self.state.focus, None)
            },
            a @ Action::Left | 
            a @ Action::Up | 
            a @ Action::Down => {
                shift_focus(self.state.focus, &self.focii, a)
            },
            Action::Route => (self.state.focus, Some(Action::ShowAnchors({
                let mut anchors: Vec<Anchor> = self.state.faust_anchors
                    .iter().enumerate().map(|(i, anchor)| Anchor {
                        index: i as u16,
                        module_id: 0,
                        name: anchor.0.clone(),
                        input: anchor.1.clone(),
                    }).collect();
                // Add a midi route if the faust plugin supports it
                if (self.state.params.iter().find(|&p| p.label == "freq").is_some() &&
                    self.state.params.iter().find(|&p| p.label == "gain").is_some() &&
                    self.state.params.iter().find(|&p| p.label == "gate").is_some()) {
                        anchors.push(Anchor {
                            index: anchors.len() as u16,
                            module_id: 0,
                            name: format!("MIDI In"),
                            input: true,
                        });
                    }
                anchors
            }))),
            a @ Action::SetParam(_,_) => (self.state.focus, Some(a)),
            _ => (self.state.focus, None)
        };

        self.state.focus = focus;

        match default {
            Some(a) => a,
            None => Action::Noop,
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
