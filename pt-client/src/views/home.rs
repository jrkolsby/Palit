use std::io::{stdout, stdin, Write, Stdout, BufReader};
use std::io::prelude::*;
use std::fs::{File};

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use crate::common::Action;
use crate::views::{Layer};

// Store for heavy, static vars
pub struct Home {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    logo_asset: String,
    state: HomeState,
}

// Store for light, cloneable vars
#[derive(Clone, Debug)]
pub struct HomeState {
    motd: String,
    projects: Vec<String>,
    focus: usize,
}

fn reduce(state: HomeState, action: Action) -> HomeState {
    let len = state.projects.len();
    let focus = match action {
        Action::Up => if state.focus == 0 { len-1 } else {
            (state.focus-1) % len
        },
        Action::Down => (state.focus+1) % len,
        _ => state.focus,
    };
    HomeState {
        motd: state.motd.clone(),
        focus: focus,
        projects: state.projects.clone(),
    }
}

impl Home {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // Load logo asset
        let asset_file = File::open("src/assets/logo.txt").unwrap();
        let mut buf_reader = BufReader::new(asset_file);
        let mut asset_str = String::new();
        buf_reader.read_to_string(&mut asset_str).unwrap();

        // Calculate center position
        let mut max_len: u16 = 0; 
        for line in asset_str.lines() {
            let len = line.len();
            if len as u16 > max_len {
                max_len = len as u16;
            }
        }

        // Initialize State
        let initial_state: HomeState = HomeState {
            motd: "It's Fun!".to_string(),
            projects: vec![
                "tinytoes.xml".to_string(),
                "heyo!!.xml".to_string(),
                "tinytoes.xml".to_string(),
                "heyo!!.xml".to_string(),
            ],
            focus: 0,
        };

        Home {
            x: x + (width / 2) - (max_len / 2),
            y: y,
            width: width,
            height: height,
            logo_asset: asset_str,
            state: initial_state
        }
    }
}

impl Layer for Home {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {


        for (i, line) in self.logo_asset.lines().enumerate() {
            write!(out, "{}{}{}",
                cursor::Goto(self.x, (i as u16)+self.y+1),
                color::Fg(color::White),
                line).unwrap();
        }

        for (i, project) in self.state.projects.iter().enumerate() {
            if (self.state.focus % self.state.projects.len()) == i {
                write!(out, "{}{}{} {} ",
                    cursor::Goto(self.x,self.y+10+(i*2) as u16),
                    color::Bg(color::Red),
                    color::Fg(color::Black),
                    project).unwrap();
            } else {
                write!(out, "{}{}{} {} ",
                    cursor::Goto(self.x,self.y+10+(i*2) as u16),
                    color::Bg(color::Reset),
                    color::Fg(color::Reset),
                    project).unwrap();
            }
        }

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::Right => {
                Action::OpenProject(self.state.projects[self.state.focus].clone())
            }
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
        false
    }
}