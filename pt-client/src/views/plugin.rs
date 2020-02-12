use std::io::Write;
use termion::cursor;
use xmltree::Element;
use libcommon::{Action, Anchor};

use crate::common::{Screen, Window};
use crate::views::{Layer};
use crate::components::{popup, ivories};
use crate::modules::param_map;

#[derive(Clone, Debug)]
struct FaustParam {
    label: String,
    init: f32,
    min: f32,
    max: f32,
    step: f32,
}

pub struct Plugin {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PluginState,
    history: Vec<PluginState>,
}

#[derive(Clone, Debug)]
pub struct PluginState {
    length: u32,
    params: Vec<FaustParam>,
}

fn reduce(state: PluginState, action: Action) -> PluginState {
    PluginState {
        length: state.length,
        params: state.params.clone(),
    }
}

impl Plugin {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Element) -> Self {
        let (_, params) = param_map(doc);
        // Initialize State
        let initial_state: PluginState = PluginState {
            length: *params.get("length").unwrap_or(&4) as u32,
            params: vec![],
        };

        Plugin {
            x: x,
            y: y,
            width: width,
            height: height,
            history: vec![],
            state: initial_state
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

        write!(out, "{}FAUST PLUGIN {}", cursor::Goto(win.x, win.y), self.state.length);
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::Route => Action::ShowAnchors(vec![Anchor {
                index: 0,
                module_id: 0,
                name: "Arp Out".to_string(),
                input: false,
            },
            Anchor {
                index: 1,
                module_id: 0,
                name: "Arp In".to_string(),
                input: true,
            }]),
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
