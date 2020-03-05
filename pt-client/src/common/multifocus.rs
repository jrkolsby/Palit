use std::io::{Write, Stdout};
use termion::{color, cursor, clear};
use libcommon::{Action};

use crate::common::{Screen, Color, write_bg, write_fg, Window};

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

pub static VOID_ID: ID = (FocusType::Void, 0);

#[derive(Clone, Debug)]
pub enum Focus {
    Red,
    Green,
    Yellow,
    Pink,
    Blue,
}

#[derive(Clone)]
pub struct MultiFocus<State> {

    // Render functions
    pub w: fn(&mut Screen, Window, ID, &State, bool),
    pub r: fn(&mut Screen, Window, ID, &State, bool),
    pub g: fn(&mut Screen, Window, ID, &State, bool),
    pub y: fn(&mut Screen, Window, ID, &State, bool),
    pub p: fn(&mut Screen, Window, ID, &State, bool),
    pub b: fn(&mut Screen, Window, ID, &State, bool),

    // Transform functions
    pub r_t: fn(Action, ID, &State) -> Action,
    pub g_t: fn(Action, ID, &State) -> Action,
    pub y_t: fn(Action, ID, &State) -> Action,
    pub p_t: fn(Action, ID, &State) -> Action,
    pub b_t: fn(Action, ID, &State) -> Action,

    // IDs
    pub w_id: ID,
    pub r_id: ID,
    pub g_id: ID,
    pub y_id: ID,
    pub p_id: ID,
    pub b_id: ID,

    pub active: Option<Focus>,
}

pub fn render_focii<T>(
    out: &mut Screen, 
    window: Window, 
    focus: (usize, usize), 
    focii: &Vec<Vec<MultiFocus<T>>>, 
    state: &T,
    light: bool,
    disable: bool) {
    let mut fullscreen = false;

    if &focii.len() <= &focus.1 || &focii[focus.1].len() <= &focus.0 {
        return;
    }

    let current_focus = &focii[focus.1][focus.0];
    if let Some(active) = &current_focus.active {
        // If something is active, fill the screen with that color
        fullscreen = true;
        match active {
            Focus::Red => { write_bg(out, Color::Red) },
            Focus::Green => { write_bg(out, Color::Green) },
            Focus::Yellow => { write_bg(out, Color::Yellow) },
            Focus::Pink => { write_bg(out, Color::Pink) },
            Focus::Blue => { write_bg(out, Color::Blue) },
        };
        write!(out, "{}", clear::All).unwrap();
        write_fg(out, Color::Black);
    } else {
        write_fg(out, if light { Color::Black } else { Color::White });
        write_bg(out, if light { Color::Beige } else { Color::Black });
    }

    for (j, col) in focii.iter().enumerate() {
        for (i, _focus) in col.iter().enumerate() {
            // Wait to render the selected focus last
            if focus == (i,j) {
                continue;
            }
            _focus.render(out, window, &state, light, false);
        }
    }

    // Render selected focus last (on top)
    current_focus.render(out, window, &state, light, !disable);

    // Default style
    if !fullscreen {
        write_fg(out, if light { Color::Black } else { Color::White }); 
        write_bg(out, if light { Color::Beige } else { Color::Black }); 
    }
}

pub fn focus_dispatch<T>(
    focus: (usize, usize), 
    focii: &mut Vec<Vec<MultiFocus<T>>>, 
    state: &T, 
    a: Action
) -> ((usize, usize), Option<Action>) {

        // Make sure focus exists
        if focus.1 >= focii.len() || focus.0 >= focii[focus.1].len() {
            return (focus, Some(a))
        }
        // Let the focus transform the action 
        let multi_focus = &mut focii[focus.1][focus.0];
        let _action = multi_focus.transform(a.clone(), state);

        match _action {
            Action::Up |
            Action::Down |
            Action::Left |
            Action::Right => shift_focus(focus, focii, _action),
            _ => (focus, Some(_action))
        }
}

pub fn shift_focus<T>(
    focus: (usize, usize), 
    focii: &Vec<Vec<MultiFocus<T>>>, 
    a: Action) ->
((usize, usize), Option<Action>) {
    // Get current focus
    if focii.len() <= focus.1 { return (focus, Some(a)) }
    let focus_row = &focii[focus.1];
    if focus_row.len() <= focus.0 { return (focus, Some(a)) }
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
    pub fn render(&self, out: &mut Screen, window: Window,
            state: &T, light: bool, focused: bool) {

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
            write_fg(out, if light { Color::White } else { Color::Black });
            write_bg(out, if light { Color::Black } else { Color::White });
            (self.w)(out, window, self.w_id.clone(), state, focused);
        } else if fullscreen {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.w)(out, window, self.w_id.clone(), state, focused);
            write_fg(out, Color::Black); 
            if full_red { write_bg(out, Color::Red) };
            if full_green { write_bg(out, Color::Green) };
            if full_yellow { write_bg(out, Color::Yellow) };
            if full_pink { write_bg(out, Color::Pink) };
            if full_blue { write_bg(out, Color::Blue) };
        } else {
            (self.w)(out, window, self.w_id.clone(), state, focused);
        }

        // Focused but not selected
        if focused && !fullscreen { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Red); 
            (self.r)(out, window, r_id, state, focused);
        // Focused and selected
        } else if full_red {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.r)(out, window, r_id, state, focused);
            write_fg(out, Color::Black); 
            write_bg(out, Color::Red); 
        // Neither focused nor selected
        } else {
            (self.r)(out, window, r_id, state, focused);
        }

        if focused && !fullscreen { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Green); 
            (self.g)(out, window, g_id, state, focused);
        } else if full_green {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.g)(out, window, g_id, state, focused);
            write_fg(out, Color::Black); 
            write_bg(out, Color::Green); 
        } else {
            (self.g)(out, window, g_id, state, focused);
        }

        if focused && !fullscreen { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Yellow); 
            (self.y)(out, window, y_id, state, focused);
        } else if full_yellow {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.y)(out, window, y_id, state, focused);
            write_fg(out, Color::Black); 
            write_bg(out, Color::Yellow); 
        } else {
            (self.y)(out, window, y_id, state, focused);
        }

        if focused && !fullscreen { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Pink); 
            (self.p)(out, window, p_id, state, focused);
        } else if full_pink {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.p)(out, window, p_id, state, focused);
            write_fg(out, Color::Black); 
            write_bg(out, Color::Pink); 
        } else {
            (self.p)(out, window, p_id, state, focused);
        }

        if focused && !fullscreen { 
            write_fg(out, Color::Black); 
            write_bg(out, Color::Blue); 
            (self.b)(out, window, b_id, state, focused);
        } else if full_blue {
            write_fg(out, Color::White); 
            write_bg(out, Color::Black); 
            (self.b)(out, window, b_id, state, focused);
            write_fg(out, Color::Black); 
            write_bg(out, Color::Blue); 
        } else {
            (self.b)(out, window, b_id, state, focused);
        }
    }

    pub fn transform(&mut self, action: Action, state: &T) -> Action {
        // Fallback to white ID if any of our ids weren't specified
        let (r_id, g_id, y_id, p_id, b_id) = (
            if self.r_id.0 == FocusType::Void { self.w_id.clone() } else { self.r_id.clone() },
            if self.g_id.0 == FocusType::Void { self.w_id.clone() } else { self.g_id.clone() },
            if self.y_id.0 == FocusType::Void { self.w_id.clone() } else { self.y_id.clone() },
            if self.p_id.0 == FocusType::Void { self.w_id.clone() } else { self.p_id.clone() },
            if self.b_id.0 == FocusType::Void { self.w_id.clone() } else { self.b_id.clone() }
        );
        match action {
            Action::SelectR => { if r_id.0 != FocusType::Void { self.active = Some(Focus::Red) }},
            Action::SelectG => { if g_id.0 != FocusType::Void { self.active = Some(Focus::Green) }},
            Action::SelectY => { if y_id.0 != FocusType::Void { self.active = Some(Focus::Yellow) }},
            Action::SelectP => { if p_id.0 != FocusType::Void { self.active = Some(Focus::Pink) }},
            Action::SelectB => { if b_id.0 != FocusType::Void { self.active = Some(Focus::Blue) }},
            Action::Deselect => { self.active = None; },
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
