use std::io::{Write, Stdout};
use termion::raw::{RawTerminal};
use xmltree::Element;

use crate::common::{MultiFocus, shift_focus, render_focii};
use crate::common::{Action, Direction, FocusType, Window, Anchor};
use crate::views::{Layer};
use crate::components::{piano, slider, button};

pub struct Piano {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PianoState,
    focii: Vec<Vec<MultiFocus<PianoState>>>,
}

#[derive(Clone, Debug)]
pub struct PianoState {
    focus: (usize, usize),
    notes: Vec<Action>,
    eq_low: i16,
    eq_mid: i16,
    eq_hi: i16,
}

fn reduce(state: PianoState, action: Action) -> PianoState {
    PianoState {
        notes: match action {
            Action::NoteOn(_,_) => { 
                let mut new_keys = state.notes.clone(); 
                new_keys.push(action);
                new_keys
            },
            Action::NoteOff(note) => {
                let mut new_keys = state.notes.clone();
                new_keys.retain(|a| match a {
                    Action::NoteOn(_note, _) => note == *_note,
                    _ => false,
                });
                new_keys
            },
            _ => state.notes.clone()
        },
        eq_low: state.eq_low,
        eq_mid: state.eq_mid,
        eq_hi: state.eq_hi,
        focus: state.focus,
    }
}

impl Piano {
    pub fn new(x: u16, y: u16, width: u16, height: u16, doc: Element) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: PianoState = PianoState {
            focus: (0,0),
            notes: vec![],
            eq_low: 8,
            eq_mid: 8,
            eq_hi: 8,
        };

        Piano {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state,
            focii: vec![vec![
                MultiFocus::<PianoState> {
                    r: |mut out, window, id, state, focus| {
                        button::render(out, window.x+2, window.y+16, 20, "Record")
                    },
                    r_t: |action, id, state| { action },
                    r_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+8, window.y+5, "20Hz".to_string(), 
                            state.eq_mid, Direction::North)
                    },
                    g_t: |action, id, state| match action {
                        Action::Up => { state.eq_mid += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_mid -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+14, window.y+5, "80Hz".to_string(), 
                            state.eq_hi, Direction::North)
                    },
                    y_t: |action, id, state| match action {
                        Action::Up => { state.eq_hi += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_hi -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+20, window.y+5, "120Hz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    p_t: |action, id, state| match action { 
                        Action::Up => { state.eq_low += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_low -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+26, window.y+5, "400Hz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    b_t: |action, id, state| match action { 
                        Action::Up => { state.eq_low += 1; Action::SetParam(0,0.0) },
                        Action::Down => { state.eq_low -= 1; Action::SetParam(0,0.0) },
                        _ => Action::Noop
                    },
                    b_id: (FocusType::Button, 0),
                    w: |mut out, window, id, state, focus| out,
                    w_id: (FocusType::Void, 0),
                    active: None,
                },
                MultiFocus::<PianoState> {
                    w: |mut out, window, id, state, focus| out,
                    w_id: (FocusType::Void, 0),
                    r: |mut out, window, id, state, focus| {
                        button::render(out, window.x+32, window.y+16, 10, "Play")
                    },
                    r_t: |action, id, state| action,
                    r_id: (FocusType::Button, 0),
                    g: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+32, window.y+5, "6KHz".to_string(), 
                            state.eq_mid, Direction::North)
                    },
                    g_t: |action, id, state| action,
                    g_id: (FocusType::Button, 0),
                    y: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+38, window.y+5, "12KHz".to_string(), 
                            state.eq_hi, Direction::North)
                    },
                    y_t: |action, id, state| action,
                    y_id: (FocusType::Button, 0),
                    p: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+44, window.y+5, "14KHz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    p_t: |action, id, state| action,
                    p_id: (FocusType::Button, 0),
                    b: |mut out, window, id, state, focus| {
                        slider::render(out, window.x+50, window.y+5, "20KHz".to_string(), 
                            state.eq_low, Direction::North)
                    },
                    b_t: |action, id, state| action,
                    b_id: (FocusType::Button, 0),
                    active: None,
                },
            ]]
        }
    }
}

impl Layer for Piano {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = piano::render(out, 
            self.x, 
            self.y, 
            &self.state.notes);

        out = render_focii(out, win, self.state.focus.clone(), &self.focii, &self.state);

        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Let the focus transform the action 
        let multi_focus = &mut self.focii[self.state.focus.1][self.state.focus.0];
        let _action = multi_focus.transform(action, &mut self.state);

        self.state = reduce(self.state.clone(), _action.clone());

        // Intercept arrow actions to change focus
        let (focus, default) = match _action {
            Action::Route => {
                (self.state.focus, Some(Action::ShowAnchors(vec![
                    Anchor {
                        id: 0, 
                        module_id: 0,
                        x: 10,
                        y: 10,
                        input: true,
                    }, Anchor {
                        id: 1, 
                        module_id: 0,
                        x: 15,
                        y: 10,
                        input: false,
                    }
                ])))
            },
            Action::Up | Action::Down | Action::Left | Action::Right => {
                shift_focus(self.state.focus, &self.focii, _action.clone())
            },
            _ => (self.state.focus, None)
        };

        self.state.focus = focus;

        match default {
            Some(a) => a,
            None => Action::Noop
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
    fn shift(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
