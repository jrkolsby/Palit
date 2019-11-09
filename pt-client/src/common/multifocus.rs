use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color, write_bg, write_fg};

const FULLWIDTH: u16 = 142;
const FULLHEIGHT: u16 = 30;

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
pub enum Focus {
    Red,
    Green,
    Yellow,
    Pink,
    Blue,
}

pub struct MultiFocus<State> {
    pub r: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub g: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub y: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub p: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub b: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub r_t: fn(Action, &mut State) -> Action,
    pub g_t: fn(Action, &mut State) -> Action,
    pub y_t: fn(Action, &mut State) -> Action,
    pub p_t: fn(Action, &mut State) -> Action,
    pub b_t: fn(Action, &mut State) -> Action,
    pub active: Option<Focus>,
}

impl<T> MultiFocus<T> {
    pub fn render(&self, mut out: RawTerminal<Stdout>, x: u16, y: u16, 
            state: &T, focused: bool) -> RawTerminal<Stdout> {
        let mut fullscreen: bool = false;
        let mut full_red: bool = false;
        let mut full_green: bool = false;
        let mut full_yellow: bool = false;
        let mut full_pink: bool = false;
        let mut full_blue: bool = false;

        if let Some(active) = &self.active {
            // If something is active, fill the screen with that color
            fullscreen = true;
            out = write_fg(match active {
                Focus::Red => { full_red = true; write_bg(out, Color::Red) },
                Focus::Green => { full_green = true; write_bg(out, Color::Green) },
                Focus::Yellow => { full_yellow = true; write_bg(out, Color::Yellow) },
                Focus::Pink => { full_pink = true; write_bg(out, Color::Pink) },
                Focus::Blue => { full_blue = true; write_bg(out, Color::Blue) },
            }, Color::Black);
            let space = (0..FULLWIDTH).map(|_| " ").collect::<String>();
            for j in 1..FULLHEIGHT {
                write!(out, "{}{}", cursor::Goto(1, j), space);
            }
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        if !fullscreen || full_red {
            out = (self.r)(out, x, y, state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Green); 
        }
        if !fullscreen || full_green {
            out = (self.g)(out, x, y, state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Yellow); 
        }
        if !fullscreen || full_yellow {
            out = (self.y)(out, x, y, state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Pink); 
        }
        if !fullscreen || full_pink {
            out = (self.p)(out, x, y, state);
        }
        if focused && !fullscreen { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Blue); 
        }
        if !fullscreen || full_blue {
            out = (self.b)(out, x, y, state);
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
            Some(Focus::Red) => (self.r_t)(action, state),
            Some(Focus::Green) => (self.g_t)(action, state),
            Some(Focus::Yellow) => (self.y_t)(action, state),
            Some(Focus::Pink) => (self.p_t)(action, state),
            Some(Focus::Blue) => (self.b_t)(action, state),
            _ => action
        }
    }
}
