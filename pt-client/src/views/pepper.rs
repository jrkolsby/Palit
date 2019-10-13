use std::io::{Write, Stdout};

use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::Action;
use crate::views::{Layer};
use crate::components::{popup, keyboard, artist};

pub struct Pepper {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PepperState,
    pepper: bool,
}

#[derive(Clone, Debug)]
pub struct PepperState {
    credits: Vec<String>,
    pepper_x: usize,
    artist_x: usize,
    snot: usize,
    score: usize,
    health: usize,
    pepper: usize,
}

fn reduce(state: PepperState, action: Action) -> PepperState {
    PepperState {
        credits: state.credits.clone(),
        pepper_x: {
            Action::Left => state.pepper_x - 1,
            Action::Right => state.pepper_x + 1,
            _ => state.pepper_x.clone(),
        }
        artist_x: match action {
            Action::Tick => state.artist_x + 1
            _ => state.artist_x.clone(),
        }
        snotState: state.snotState.clone(),
        score: state.score.clone(),
        health: state.health.clone(),
        pepper: match action {
            Action::SelectR => !state.pepper,
            _ => state.pepper.clone(),
        }
    }
}

impl Pepper {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        // Initialize State
        let initial_state: PepperState = PepperState {
            credits: vec![
                "Designed by James Kolsby".to_string(), 
                "ASCII art by".to_string(), 
                "asciiart.com".to_string()],
            // 6 possible position states for both actors
            pepper_x: 0,
            artist_x: 4,
            // 5 possible states, sneeze is the 5th state
            snotState: 0,
            score: 0,
            health: 5,
        };

        Pepper {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Pepper {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        out = artist::render(self.x, self.y, self.state.artist_x);

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
        false
    }
}
