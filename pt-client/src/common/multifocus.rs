use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color, write_bg, write_fg};

/*
    A multifocus render will render five components prefixed with
    a write_bg and write_fg method. Note that this will require making
    components bg and fg independent. A view will keep a 2D array of 
    MultiFocus objects which will be navigated when the view receives
    Up Down Left Right actions. If we reach the border of a list and try
    to exceed it, the view should pass a default action back up to the
    parent containing the direction to navigate. 
*/

pub enum Focus {
    Red,
    Pink,
    Green,
    Blue,
    Yellow,
}

pub struct FocusState {
    pub focus: (usize, usize),
    pub active: Focus,
}

pub struct MultiFocus<State> {
    pub r: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub g: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub y: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub p: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
    pub b: fn(RawTerminal<Stdout>, u16, u16, &State) -> RawTerminal<Stdout>,
}

impl<T> MultiFocus<T> {
    pub fn render(&self, mut out: RawTerminal<Stdout>, x: u16, y: u16, state: &T, active: bool) -> RawTerminal<Stdout> {
        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Red); 
        }
        out = (self.r)(out, x, y, state);

        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Green); 
        }
        out = (self.g)(out, x, y, state);

        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Yellow); 
        }
        out = (self.y)(out, x, y, state);
        
        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Pink); 
        }
        out = (self.p)(out, x, y, state);

        if active { 
            out = write_fg(out, Color::Black); 
            out = write_bg(out, Color::Blue); 
        }
        out = (self.b)(out, x, y, state);
        out = write_fg(out, Color::White); 
        out = write_bg(out, Color::Transparent); 
        out
    }
}
