use std::io::Write;
use termion::cursor;
use libcommon::{Action, Anchor, Module};

use crate::common::{Screen, Direction, FocusType, Window};
use crate::common::{MultiFocus, ID, focus_dispatch, render_focii};
use crate::components::{popup};
use crate::views::{Layer};

static PADDING: (u16, u16) = (3, 3);

pub struct Project {
    window: Window,
    state: ProjectState,
    focii: Vec<Vec<MultiFocus<ProjectState>>>,
}

#[derive(Clone, Debug)]
pub struct ProjectState {
    title: String,
    modules: Vec<Module>,
    focus: (usize, usize),
}

impl Project {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        let initial_state = ProjectState {
            focus: (0,0),
            title: "".to_string(),
            modules: vec![],
        };
        return Project {
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

static VOID_RENDER: fn( &mut Screen, Window, ID, &ProjectState, bool) =
    |_, _, _, _, _| {};

fn reduce(state: ProjectState, action: Action) -> ProjectState {
    ProjectState {
        focus: state.focus,
        title: match action.clone() {
            Action::ShowProject(title, _) => title,
            _ => state.title.clone()
        },
        modules: match action {
            Action::ShowProject(_, modules) => modules,
            _ => state.modules.clone()
        },
    }
}

fn generate_focii(modules: &Vec<Module>) -> Vec<Vec<MultiFocus<ProjectState>>> {
    let mut focii = vec![];

    let mut sorted_modules = modules.clone();
    sorted_modules.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for (i, module) in sorted_modules.iter().enumerate() {
        focii.push(vec![MultiFocus::<ProjectState> {
            w_id: (FocusType::Button, i as u16),
            w: |mut out, window, id, state, focus| {
                let displayName = &state.modules[id.1 as usize].name;
                write!(out, "{}{}", cursor::Goto(
                    PADDING.0 + window.x, 
                    PADDING.1 + window.y + id.1 * 2,
                ), displayName).unwrap();
            },
            r_id: (FocusType::Void, 0),
            r: |mut out, window, id, state, focus| {
                if focus {
                    let displayName = &state.modules[id.1 as usize].name;
                    let displayLen = displayName.len() as u16 + 1;
                    write!(out, "{}DELETE", cursor::Goto(
                        PADDING.0 + window.x + displayLen,
                        PADDING.1 + window.y + id.1 * 2,
                    )).unwrap()
                }
            },
            r_t: |a, id, state| match a {
                Action::SelectR |
                Action::SelectG |
                Action::SelectP |
                Action::SelectY |
                Action::SelectB => Action::DelModule(state.modules[id.1 as usize].id),
                _ => Action::Noop
            },
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
        }])
    }
    focii
}

impl Layer for Project {
    fn render(&self, out: &mut Screen, target: bool) {
        popup::render(out, 
                      self.window.x,
                      self.window.y,
                      self.window.w,
                      self.window.h,
                      &self.state.title);
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
            self.state = reduce(self.state.clone(), _default.clone());
            match action {
                Action::ShowProject(_,_) => {
                    self.focii = generate_focii(&self.state.modules);
                },
                _ => {}
            };
            match _default {
                Action::Back => Action::Cancel,
                a @ Action::DelModule(_) => a,
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