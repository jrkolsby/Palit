use std::io::{stdout, stdin, Write, Stdout, BufReader};
use std::io::prelude::*;
use std::fs::{File};

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{RawTerminal};

pub struct Home {
    logo_asset: String,
    logo_x: u16,
    logo_y: u16,
    state: HomeState,
}

#[derive(Clone, Debug)]
pub struct HomeState {
    motd: String,
    projects: Vec<String>,
    focus: usize,
}

pub enum HomeAction {
    Up,
    Down,
    Select,
    Noop,
}

fn reduce(state: HomeState, action: HomeAction) -> HomeState {
    let len = state.projects.len();
    HomeState {
        motd: state.motd.clone(),
        focus: match action {
            HomeAction::Up => if state.focus < 2 { len } else {
                (state.focus-1) % len
            },
            HomeAction::Down => (state.focus+1) % len,
            _ => state.focus,
        },
        projects: state.projects.clone(),
    }
}

impl Home {
    pub fn new() -> Self {
        // Load logo asset
        let asset_file = File::open("src/assets/logo.txt").unwrap();
        let mut buf_reader = BufReader::new(asset_file);
        let mut asset_str = String::new();
        buf_reader.read_to_string(&mut asset_str).unwrap();

        // Calculate center position
        let size: (u16, u16) = terminal_size().unwrap();
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
            focus: 100,
        };

        Home {
            logo_asset: asset_str,
            logo_x: (size.0 / 2) - (max_len / 2),
            logo_y: 3,
            state: initial_state
        }
    }

    pub fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {


        for (i, line) in self.logo_asset.lines().enumerate() {
            write!(out, "{}{}{}",
                cursor::Goto(self.logo_x, (i as u16)+self.logo_y+1),
                color::Fg(color::White),
                line).unwrap();
        }

        for (i, project) in self.state.projects.iter().enumerate() {
            if (self.state.focus % self.state.projects.len()) == i {
                write!(out, "{}{}{} {} ",
                    cursor::Goto(self.logo_x,self.logo_y+10+(i*2) as u16),
                    color::Bg(color::Red),
                    color::Fg(color::Black),
                    project).unwrap();
            } else {
                write!(out, "{}{}{} {} ",
                    cursor::Goto(self.logo_x,self.logo_y+10+(i*2) as u16),
                    color::Bg(color::Reset),
                    color::Fg(color::Reset),
                    project).unwrap();
            }
        }

        write!(out, "{}", color::Bg(color::Reset)).unwrap();
        out.flush().unwrap();

        out
    }

    pub fn dispatch(&mut self, action: HomeAction) {
        self.state = reduce(self.state.clone(), action);
    }
}