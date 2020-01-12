use std::io::Write;
use termion::cursor;

use crate::common::{Screen, Action, Direction, FocusType, Window, Anchor};
use crate::common::{MultiFocus, ID, focus_dispatch};
use crate::common::{get_files, PALIT_MODULES};
use crate::views::{Layer};

pub struct Modules {
    state: ModulesState,
    focii: Vec<Vec<MultiFocus<ModulesState>>>,
}

#[derive(Clone, Debug)]
pub struct ModulesState {
    modules: Vec<String>,
    focus: (usize, usize),
}

impl Modules {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        let modules = get_files(PALIT_MODULES, vec![]);
        let initial_state = ModulesState {
            focus: (0,0),
            modules: modules.unwrap(),
        };
        return Modules {
            focii: generate_focii(&initial_state.modules),
            state: initial_state,
        }
    }
}

static VOID_RENDER: fn( &mut Screen, Window, ID, &ModulesState, bool) =
    |_, _, _, _, _| {};

fn reduce(state: ModulesState, action: Action) -> ModulesState {
    return state.clone();
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
    let mut focii = vec![];
    focii
}

impl Layer for Modules {
    fn render(&self, out: &mut Screen, target: bool) {
        write!(out, "HI");
    }

    fn dispatch(&mut self, action: Action) -> Action {
        let (focus, default) = focus_dispatch(self.state.focus, 
                                              &mut self.focii, 
                                              &self.state, 
                                              action.clone());
        self.state.focus = focus;

        if let Some(_default) = default {
            self.state = reduce(self.state.clone(), _default.clone());
            match _default {
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
        false
    }

}