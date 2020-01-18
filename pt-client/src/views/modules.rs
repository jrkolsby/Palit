use std::io::Write;
use termion::cursor;

use crate::common::{Screen, Action, Direction, FocusType, Window, Anchor};
use crate::common::{MultiFocus, ID, focus_dispatch, render_focii};
use crate::common::{get_files, PALIT_MODULES, Module};
use crate::components::{popup};
use crate::views::{Layer};

static PADDING: (u16, u16) = (3, 3);

const CORE_MODULES: [&str; 4] = [
    "timeline",
    "hammond",
    "arpeggio",
    "keyboard",
];

/*
struct Module {
    name: String,
    src: String,
    icon: String,
}
*/

pub struct Modules {
    window: Window,
    state: ModulesState,
    focii: Vec<Vec<MultiFocus<ModulesState>>>,
}

#[derive(Clone, Debug)]
pub struct ModulesState {
    modules: Vec<String>,
    current: Option<usize>,
    focus: (usize, usize),
}

impl Modules {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        let mut core: Vec<String> = CORE_MODULES.iter().map(|a| a.to_string()).collect();
        let modules = get_files(PALIT_MODULES, core);
        let initial_state = ModulesState {
            focus: (0,0),
            current: None,
            modules: modules.unwrap(),
        };
        return Modules {
            window: Window {
                x, y,
                w: width,
                h: height,
            },
            focii: generate_focii(&initial_state.modules),
            state: initial_state,
        }
    }
}

static VOID_RENDER: fn( &mut Screen, Window, ID, &ModulesState, bool) =
    |_, _, _, _, _| {};

fn reduce(state: ModulesState, action: Action) -> ModulesState {
    ModulesState {
        modules: state.modules.clone(),
        current: match action {
            Action::TryoutModule(id) => Some(id as usize),
            Action::Deselect => None,
            _ => state.current,
        },
        focus: state.focus,
    }
}

fn generate_focii(modules: &Vec<String>) -> Vec<Vec<MultiFocus<ModulesState>>> {
    let void_focus = MultiFocus::<ModulesState> {
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

    let mut sorted_modules = modules.clone();
    sorted_modules.sort();

    for (i, module) in sorted_modules.iter().enumerate() {
        let id = (FocusType::Button, i as u16);
        let transform: fn(Action, ID, &ModulesState) -> Action = |a, id, state| match a {
            Action::SelectR |
            Action::SelectG |
            Action::SelectP |
            Action::SelectY |
            Action::SelectB => Action::TryoutModule(id.1),
            _ => a
        };
        let render: fn(&mut Screen, Window, ID, &ModulesState, bool) = 
            |mut out, window, id, state, focus| {
                let module = &state.modules[id.1 as usize];
                write!(out, "{}{}", cursor::Goto(
                    PADDING.0 + window.x, 
                    PADDING.1 + window.y + id.1 * 2,
                ), module).unwrap();
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

impl Layer for Modules {
    fn render(&self, out: &mut Screen, target: bool) {
        popup::render(out, 
                      self.window.x,
                      self.window.y,
                      self.window.w,
                      self.window.h,
                      &"Add Module".to_string());
        render_focii(out, self.window, 
            self.state.focus.clone(), 
            &self.focii, &self.state, true, !target);
    }

    fn dispatch(&mut self, action: Action) -> Action {
        let (focus, default) = focus_dispatch(self.state.focus, 
                                              &mut self.focii, 
                                              &self.state, 
                                              action.clone());
        self.state.focus = focus;

        if let Some(_default) = default {
            let current = self.state.current;
            self.state = reduce(self.state.clone(), _default.clone());
            match _default {
                Action::Deselect => if let Some(i) = current {
                    Action::AddModule(self.state.modules[i].clone())
                } else { Action::Cancel },
                Action::Back => Action::Cancel,
                _ => Action::Noop,
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
        true
    }

}