use std::io::{stdout, stdin, Write, Stdout, BufReader};
use std::io::prelude::*;
use std::fs::{File};

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use crate::common::Action;
use crate::views::{Layer};

pub struct Help {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    keyboard_asset: String,
    state: HelpState,
}

#[derive(Clone, Debug)]
pub struct HelpState {
    active: Vec<Action>
}

fn reduce(state: HelpState, action: Action) -> HelpState {
    HelpState {
        active: state.active.clone()
    }
}

impl Help {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // Load keyboard asset
        let asset_file = File::open("src/assets/keyboard.txt").unwrap();
        let mut buf_reader = BufReader::new(asset_file);
        let mut asset_str = String::new();
        buf_reader.read_to_string(&mut asset_str).unwrap();

        // Initialize State
        let initial_state: HelpState = HelpState {
            active: vec![Action::Noop]
        };

        Help {
            x: x,
            y: y,
            width: width,
            height: height,
            keyboard_asset: asset_str,
            state: initial_state
        }
    }
}

impl Layer for Help {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        for x in 0..self.width {
            for y in 0..self.height {
                write!(out, "{}{} ",
                    cursor::Goto(self.x+x, self.y+y),
                    color::Bg(color::LightYellow)).unwrap();
                if ((x+1) % self.width == 0 ||
                    (y+1) % self.height == 0) {
                    write!(out, "{}{}  ",
                        cursor::Goto(self.x+x+1, self.y+y+1),
                        color::Bg(color::LightBlue)).unwrap();
                }
            }
        }

        write!(out, "{}", color::Bg(color::LightYellow)).unwrap();

        for (i, line) in self.keyboard_asset.lines().enumerate() {
            write!(out, "{}{}{}",
                cursor::Goto(self.x+5, (i as u16)+self.y+5),
                color::Fg(color::Black),
                line).unwrap();
        }

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