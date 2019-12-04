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

#[derive(PartialEq)]
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

    // Render functions
    pub w: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,
    pub r: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,
    pub g: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,
    pub y: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,
    pub p: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,
    pub b: fn(RawTerminal<Stdout>, Window, ID, &State, bool) -> RawTerminal<Stdout>,

    // Transform functions
    pub r_t: fn(Action, ID, &mut State) -> Action,
    pub g_t: fn(Action, ID, &mut State) -> Action,
    pub y_t: fn(Action, ID, &mut State) -> Action,
    pub p_t: fn(Action, ID, &mut State) -> Action,
    pub b_t: fn(Action, ID, &mut State) -> Action,

    // IDs
    pub w_id: ID,
    pub r_id: ID,
    pub g_id: ID,
    pub y_id: ID,
    pub p_id: ID,
    pub b_id: ID,

    pub active: Option<Focus>,
}

pub fn render_focii<T>(mut out: RawTerminal<Stdout>, window: Window, 
        focus: (usize, usize), focii: &Vec<Vec<MultiFocus<T>>>, state: &T) 
        -> RawTerminal<Stdout> {
    let mut fullscreen = false;

    let current_focus = &focii[focus.1][focus.0];
    if let Some(active) = &current_focus.active {
        // If something is active, fill the screen with that color
        let size: (u16, u16) = terminal_size().unwrap(); 
        fullscreen = true;
        out = match active {
            Focus::Red => { write_bg(out, Color::Red) },
            Focus::Green => { write_bg(out, Color::Green) },
            Focus::Yellow => { write_bg(out, Color::Yellow) },
            Focus::Pink => { write_bg(out, Color::Pink) },
            Focus::Blue => { write_bg(out, Color::Blue) },
        };
        let space = (0..size.0).map(|_| " ").collect::<String>();
        for j in 1..size.1 {
            write!(out, "{}{}", cursor::Goto(1, j), space);
        }
        out = write_fg(out, Color::Black);
    } else {
        out = write_fg(out, Color::White);
        out = write_bg(out, Color::Black);
    }

    for (j, col) in focii.iter().enumerate() {
        for (i, _focus) in col.iter().enumerate() {
            // Wait to render the selected focus last
            if focus == (i,j) {
                continue;
            }
            out = _focus.render(out, window, &state, false);
        }
    }

    // Render selected focus last (on top)
    out = current_focus.render(out, window, &state, true);

    // Default style
    if !fullscreen {
        out = write_fg(out, Color::White); 
        out = write_bg(out, Color::Black); 
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
                } else { focus },
            Action::Right => if focus.0 < (focus_row.len()-1) {
                    (focus.0+1, focus.1)
                } else { focus },
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

        let mut fullscreen: bool = false;
        let mut full_red: bool = false;
        let mut full_green: bool = false;
        let mut full_yellow: bool = false;
        let mut full_pink: bool = false;
        let mut full_blue: bool = false;

        if let Some(active) = &self.active {
            fullscreen = true;
            match active {
                Focus::Red => { full_red = true; },
                Focus::Green => { full_green = true; },
                Focus::Yellow => { full_yellow = true; },
                Focus::Pink => { full_pink = true; },
                Focus::Blue => { full_blue = true; },
            };
        }

        // Fallback to white id if any ids weren't specified
        let (r_id, g_id, y_id, p_id, b_id) = (
            if self.r_id.0 == FocusType::Void { self.w_id.clone() } else { self.r_id.clone() },
            if self.g_id.0 == FocusType::Void { self.w_id.clone() } else { self.g_id.clone() },
            if self.y_id.0 == FocusType::Void { self.w_id.clone() } else { self.y_id.clone() },
            if self.p_id.0 == FocusType::Void { self.w_id.clone() } else { self.p_id.clone() },
            if self.b_id.0 == FocusType::Void { self.w_id.clone() } else { self.b_id.clone() }
        );

        if focused && !fullscreen {
            out = write_fg(out, Color::Black);
            out = write_bg(out, Color::White);
            out = (self.w)(out, window, self.w_id.clone(), state, focused);
        } else if fullscreen {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.w)(out, window, self.w_id.clone(), state, focused);
            out = write_fg(out, Color::Black); 
            if full_red { out = write_bg(out, Color::Red) };
            if full_green { out = write_bg(out, Color::Green) };
            if full_yellow { out = write_bg(out, Color::Yellow) };
            if full_pink { out = write_bg(out, Color::Pink) };
            if full_blue { out = write_bg(out, Color::Blue) };
        } else {
            out = (self.w)(out, window, self.w_id.clone(), state, focused);
        }

        // Focused but not selected
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
            out = (self.r)(out, window, r_id, state, focused);
        // Focused and selected
        } else if full_red {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.r)(out, window, r_id, state, focused);
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        // Neither focused nor selected
        } else {
            out = (self.r)(out, window, r_id, state, focused);
        }

        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Green); 
            out = (self.g)(out, window, g_id, state, focused);
        } else if full_green {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.g)(out, window, g_id, state, focused);
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Green); 
        } else {
            out = (self.g)(out, window, g_id, state, focused);
        }

        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Yellow); 
            out = (self.y)(out, window, y_id, state, focused);
        } else if full_yellow {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.y)(out, window, y_id, state, focused);
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Yellow); 
        } else {
            out = (self.y)(out, window, y_id, state, focused);
        }

        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Pink); 
            out = (self.p)(out, window, p_id, state, focused);
        } else if full_pink {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.p)(out, window, p_id, state, focused);
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Pink); 
        } else {
            out = (self.p)(out, window, p_id, state, focused);
        }

        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Blue); 
            out = (self.b)(out, window, b_id, state, focused);
        } else if full_blue {
            out = write_fg(out, Color::White); 
            out = write_bg(out, Color::Black); 
            out = (self.b)(out, window, b_id, state, focused);
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Blue); 
        } else {
            out = (self.b)(out, window, b_id, state, focused);
        }

        out
    }
    pub fn transform(&mut self, action: Action, state: &mut T) -> Action {
        // Fallback to white ID if any of our ids weren't specified
        let (r_id, g_id, y_id, p_id, b_id) = (
            if self.r_id.0 == FocusType::Void { self.w_id.clone() } else { self.r_id.clone() },
            if self.g_id.0 == FocusType::Void { self.w_id.clone() } else { self.g_id.clone() },
            if self.y_id.0 == FocusType::Void { self.w_id.clone() } else { self.y_id.clone() },
            if self.p_id.0 == FocusType::Void { self.w_id.clone() } else { self.p_id.clone() },
            if self.b_id.0 == FocusType::Void { self.w_id.clone() } else { self.b_id.clone() }
        );
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
            Some(Focus::Red) => (self.r_t)(action, r_id, state),
            Some(Focus::Green) => (self.g_t)(action, g_id, state),
            Some(Focus::Yellow) => (self.y_t)(action, y_id, state),
            Some(Focus::Pink) => (self.p_t)(action, p_id, state),
            Some(Focus::Blue) => (self.b_t)(action, b_id, state),
            _ => action
        }
    }
}
