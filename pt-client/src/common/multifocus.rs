use std::io::{Write, Stdout};
use termion::{color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color, write_bg, write_fg, Window};

/*
    A multifocus render will render five components prefixed with
    a write_bg and write_fg method. Note that this will require making
    components bg and fg independent. A view will keep a 2D array of 
    MultiFocus objects which will be navigated when the view receives
    Up Down Left Right actions. If we reach the border of a list and try
    to exceed it, the view should pass a default action back up to the
    parent containing the direction to navigate. 
*/

#[derive(Clone, Debug)]
pub enum FocusType {
    Param,
    Region,
    Button,
    Switch,
    Input,
    Output,
    Void,
}

pub type ID = (FocusType, u16);

#[derive(Clone, Debug)]
pub enum Focus {
    Red,
    Green,
    Yellow,
    Pink,
    Blue,
}

pub struct MultiFocus<State> {
    pub r: fn(RawTerminal<Stdout>, Window, ID, &State) -> RawTerminal<Stdout>,
    pub g: fn(RawTerminal<Stdout>, Window, ID, &State) -> RawTerminal<Stdout>,
    pub y: fn(RawTerminal<Stdout>, Window, ID, &State) -> RawTerminal<Stdout>,
    pub p: fn(RawTerminal<Stdout>, Window, ID, &State) -> RawTerminal<Stdout>,
    pub b: fn(RawTerminal<Stdout>, Window, ID, &State) -> RawTerminal<Stdout>,
    pub r_t: fn(Action, ID, &mut State) -> Action,
    pub g_t: fn(Action, ID, &mut State) -> Action,
    pub y_t: fn(Action, ID, &mut State) -> Action,
    pub p_t: fn(Action, ID, &mut State) -> Action,
    pub b_t: fn(Action, ID, &mut State) -> Action,
    pub active: Option<Focus>,
    pub r_id: ID,
    pub g_id: ID,
    pub y_id: ID,
    pub p_id: ID,
    pub b_id: ID,
}

pub fn render_focii<T>(mut out: RawTerminal<Stdout>, window: Window, 
        focus: (usize, usize), focii: &Vec<Vec<MultiFocus<T>>>, state: &T) 
        -> RawTerminal<Stdout> {
    if let Some(_) = focii[focus.1][focus.0].active {
        out = focii[focus.1][focus.0].render(out, window, &state, true);
    } else {
        for (j, col) in focii.iter().enumerate() {
            for (i, _focus) in col.iter().enumerate() {
                out = _focus.render(out, window, &state, focus == (i,j));
            }
        }
    }
    out
}

pub fn shift_focus<T>(focus: (usize, usize), focii: &Vec<Vec<MultiFocus<T>>>, a: Action) -> 
        ((usize, usize), Option<Action>) {
    let focus_row = &focii[focus.1];
    let focus_i = &focus_row[focus.0];
    let mut default: Option<Action> = None;
    let mut focus = focus.clone();
    if focus_i.active.is_none() {
        focus = match a {
            Action::Left => if focus.0 > 0 {
                    (focus.0-1, focus.1)
                // If the user tried to exceed the focus bounds, pass default back up 
                // to the caller 
                } else { default = Some(Action::Left); focus },
            Action::Right => if focus.0 < (focus_row.len()-1) {
                    (focus.0+1, focus.1)
                } else { default = Some(Action::Right); focus },
            Action::Up => if focus.1 > 0 {
                    let row_up = &focii[focus.1-1];
                    if focus.0 >= row_up.len()-1 {
                        // Go to end of row if its shorter than current row
                        (row_up.len()-1, focus.1-1)
                    } else {
                        (focus.0, focus.1-1)
                    }
                } else { default = Some(Action::Up); focus }
            Action::Down => if focus.1 < (focii.len()-1) {
                    let row_down = &focii[focus.1+1];
                    if focus.0 >= row_down.len()-1 {
                        (row_down.len()-1, focus.1+1)
                    } else {
                        (focus.0, focus.1+1)
                    }
                } else { default = Some(Action::Down); focus },
            _ => focus
        };
    }
    return (focus, default);

}

impl<T> MultiFocus<T> {
    pub fn render(&self, mut out: RawTerminal<Stdout>, window: Window,
            state: &T, focused: bool) -> RawTerminal<Stdout> {

        out = write_fg(out, Color::White); 
        out = write_bg(out, Color::Transparent); 

        let mut fullscreen: bool = false;
        let mut full_red: bool = false;
        let mut full_green: bool = false;
        let mut full_yellow: bool = false;
        let mut full_pink: bool = false;
        let mut full_blue: bool = false;

        if let Some(active) = &self.active {
            let size: (u16, u16) = terminal_size().unwrap(); 
            // If something is active, fill the screen with that color
            fullscreen = true;
            out = write_fg(match active {
                Focus::Red => { full_red = true; write_bg(out, Color::Red) },
                Focus::Green => { full_green = true; write_bg(out, Color::Green) },
                Focus::Yellow => { full_yellow = true; write_bg(out, Color::Yellow) },
                Focus::Pink => { full_pink = true; write_bg(out, Color::Pink) },
                Focus::Blue => { full_blue = true; write_bg(out, Color::Blue) },
            }, Color::Black);
            let space = (0..size.0).map(|_| " ").collect::<String>();
            for j in 1..size.1 {
                write!(out, "{}{}", cursor::Goto(1, j), space);
            }
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        if !fullscreen || full_red {
            out = (self.r)(out, window, self.r_id.clone(), state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Green); 
        }
        if !fullscreen || full_green {
            out = (self.g)(out, window, self.g_id.clone(), state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Yellow); 
        }
        if !fullscreen || full_yellow {
            out = (self.y)(out, window, self.y_id.clone(),  state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Pink); 
        }
        if !fullscreen || full_pink {
            out = (self.p)(out, window, self.p_id.clone(), state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Blue); 
        }
        if !fullscreen || full_blue {
            out = (self.b)(out, window, self.b_id.clone(), state);
        }

        // Default style
        out = write_fg(out, Color::White); 
        out = write_bg(out, Color::Transparent); 
        out
    }
    pub fn transform(&mut self, action: Action, state: &mut T) -> Action {
        match action {
            Action::SelectR => { self.active = Some(Focus::Red) },
            Action::SelectG => { self.active = Some(Focus::Green) },
            Action::SelectY => { self.active = Some(Focus::Yellow) },
            Action::SelectP => { self.active = Some(Focus::Pink) },
            Action::SelectB => { self.active = Some(Focus::Blue) },
            Action::Deselect => { self.active = None },
            _ => {},
        };
        match self.active {
            Some(Focus::Red) => (self.r_t)(action, self.r_id.clone(), state),
            Some(Focus::Green) => (self.g_t)(action, self.g_id.clone(), state),
            Some(Focus::Yellow) => (self.y_t)(action, self.y_id.clone(), state),
            Some(Focus::Pink) => (self.p_t)(action, self.p_id.clone(), state),
            Some(Focus::Blue) => (self.b_t)(action, self.b_id.clone(), state),
            _ => action
        }
    }
}
