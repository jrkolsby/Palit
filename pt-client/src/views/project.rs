use std::io::Write;
use termion::cursor;

use crate::common::{Screen, Action, Direction, FocusType, Window, Anchor};
use crate::common::{MultiFocus, ID, focus_dispatch, render_focii, Color, write_bg, write_fg};
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
    modules: Vec<String>,
    title: String,
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

fn generate_focii(modules: &Vec<String>) -> Vec<Vec<MultiFocus<ProjectState>>> {
    let void_focus = MultiFocus::<ProjectState> {
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
        let transform: fn(Action, ID, &ProjectState) -> Action = |a, id, state| match a {
            Action::SelectR |
            Action::SelectG |
            Action::SelectP |
            Action::SelectY |
            Action::SelectB => Action::AddModule(state.modules[id.1 as usize].clone()),
            _ => Action::Noop
        };
        let render: fn(&mut Screen, Window, ID, &ProjectState, bool) = 
            |mut out, window, id, state, focus| {
                let module = &state.modules[id.1 as usize];
                if !focus { write_bg(out, Color::Beige); write_fg(out, Color::Black); }
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