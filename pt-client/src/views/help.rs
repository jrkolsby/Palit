use std::io::{stdout, stdin, Write, Stdout, BufReader};
use std::io::prelude::*;
use std::fs::{File};

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use crate::common::Action;
use crate::views::{Layer};
use crate::components::{keyboard};

pub struct Help {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: HelpState,
}

#[derive(Clone, Debug)]
pub struct HelpState {
    title: String,
    active: Vec<Action>
}

fn reduce(state: HelpState, action: Action) -> HelpState {
    HelpState {
        title: state.title.clone(),
        active: state.active.clone()
    }
}

impl Help {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // Initialize State
        let initial_state: HelpState = HelpState {
            title: "Please Help Me Please".to_string(),
            active: vec![Action::Noop]
        };

        Help {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Help {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        for x in 0..self.width {
            for y in 0..self.height {
                let left = x == 0;
                let top = y == 0;
                let right = x == self.width-1;
                let bottom = y == self.height-1;
                write!(out, "{}{}{}{}",
                    cursor::Goto(self.x+x, self.y+y),
                    color::Fg(color::Black),
                    color::Bg(color::LightYellow),
                    match (top, right, bottom, left){
                        // TOP LEFT
                        (true, false, false, true) => "┌",
                        (false, true, true, false) => "┘",
                        (true, true, false, false) => "┐",
                        (false, false, true, true) => "└",
                        (false, false, false, true) => "│",
                        (false, true, false, false) => "│",
                        (true, false, false, false) => "─",
                        (false, false, true, false) => "─",
                        _ => " "
                    }).unwrap();
                if (right || bottom) {
                    write!(out, "{}{}  ",
                        cursor::Goto(self.x+x+1, self.y+y+1),
                        color::Bg(color::LightBlue)).unwrap();
                }
                let title_len = self.state.title.len() as u16;
                let title_x = (self.width/2) - (title_len/2);
                write!(out, "{}{}{} {} ",
                    cursor::Goto(self.x+title_x, self.y),
                    color::Bg(color::LightYellow),
                    color::Fg(color::Black),
                    self.state.title).unwrap();
            }
        }

        out = keyboard::render(out, self.x+5, self.y+5);

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::SelectR => Action::Back,
            _ => Action::Noop
        }
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