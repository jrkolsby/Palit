use std::io::{stdout, stdin, Write, Stdout, BufReader};
use std::io::prelude::*;
use std::fs::{File};

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color};
use crate::views::{Layer};
use crate::components::{logo, button};

const NUM_FOCII: usize = 3;
const NUM_PROJECTS: usize = 4;

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
    scroll_x: usize,
}

fn reduce(state: HomeState, action: Action) -> HomeState {
    let len = state.projects.len();
    let scroll_max = state.projects.len()/4;
    let scroll_x = match action {
        Action::Left => {
            eprintln!("HOME LEFT");
            if state.scroll_x == 0 { scroll_max-1 } else {
                (state.scroll_x-1) % scroll_max
            }
        },
        Action::Right => (state.scroll_x+1) % scroll_max,
        _ => state.scroll_x,
    };
    let focus = match action {
	Action::Up => if state.focus == 0 { NUM_FOCII-1 } else {
            (state.focus-1) % NUM_FOCII
        },
	Action::Down => (state.focus+1) % NUM_FOCII,
	_ => state.focus
    };
    HomeState {
        motd: state.motd.clone(),
        projects: state.projects.clone(),
        focus,
	scroll_x,
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
                "one.xml".to_string(),
                "two.xml".to_string(),
                "three.xml".to_string(),
                "four.xml".to_string(),
                "five.xml".to_string(),
                "six.xml".to_string(),
                "seven.xml".to_string(),
                "eight.xml".to_string(),
                "tinytoes.xml".to_string(),
                "heyo!!.xml".to_string(),
                "tinytoes.xml".to_string(),
                "heyo!!.xml".to_string(),
            ],
            focus: 0,
	    scroll_x: 0,
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

	// Logo
        out = logo::render(out, self.x, self.y);

	// New Button
	out = button::render(out, self.x + 10, self.y + 10, 17, 
        "New Project", Color::Red, self.state.focus == 0);

	// Project Listing
	let mut col: [u16; 2] = [4,4];
        for (i, project) in self.state.projects.iter().enumerate() {
	    if (i >= self.state.scroll_x * NUM_PROJECTS && 
		i < (self.state.scroll_x+1) * NUM_PROJECTS) {
		let j: u16 = (i % NUM_PROJECTS) as u16;
		let row: usize = (j % 2) as usize;
		write!(out, "{}", cursor::Goto(self.x+col[row] as u16, 
		    self.y+15+(row as u16 * 2))).unwrap();
		if self.state.focus == 0 { 
		    write!(out, "{}", color::Fg(color::Black)).unwrap();
		    match j {
			0 => write!(out, "{}", color::Bg(color::Yellow)).unwrap(),
			1 => write!(out, "{}", color::Bg(color::Magenta)).unwrap(),
			2 => write!(out, "{}", color::Bg(color::Blue)).unwrap(),
			3 => write!(out, "{}", color::Bg(color::Green)).unwrap(),
			_ => write!(out, "{}", color::Bg(color::Reset)).unwrap(), 
		    }
		} else {
		    write!(out, "{}{}", 
			color::Fg(color::White), 
			color::Bg(color::Reset)
		    ).unwrap();
		}

		write!(out, " {} ", project).unwrap();
		col[row] += project.len() as u16 + 4;
	    }
        }

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());
        match action {
            Action::SelectR => { Action::CreateProject }
            Action::SelectY => {
                eprintln!("{} {}", self.state.projects.len(), self.state.scroll_x);
                Action::OpenProject(self.state.projects[self.state.scroll_x].clone())
            },
            Action::Up => { 
                if self.state.focus == 2 { Action::Pepper } else { Action::Noop }
            },
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
