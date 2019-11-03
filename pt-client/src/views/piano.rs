use std::io::{Write, Stdout};
use termion::{color, cursor};
use termion::raw::{RawTerminal};

use crate::common::{Action, Color};
use crate::views::{Layer};
use crate::components::{upright, knob, slider};

pub struct Piano {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: PianoState,
}

#[derive(Clone, Debug)]
pub struct PianoState {
    notes: Vec<Action>
}

const ASCII_MAX: u8 = 126;
const ASCII_MIN: u8 = 48;

fn reduce(state: PianoState, action: Action) -> PianoState {
    PianoState {
        keys: match action {
            Action::NoteOn(_,_) => { 
                let new_keys = state.keys.clone(); 
                new_keys.push(action);
                new_keys
            },
            Action::NoteOff(a,_) => {
                state.keys.retain(|a| match a {
                    Action::NoteOn(_a, ) => a == _a,
                    _ => false,
                })
            }
        }

    }
}

impl Title {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        let mut path: String = "/usr/local/palit/".to_string();

        // Initialize State
        let initial_state: TitleState = TitleState {
            letter: ASCII_MIN,
            title_val: path,
            title: "What's it called?".to_string(),
        };

        Title {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Title {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {
        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

	    out = popup::render(out, self.x, self.y, self.width, self.height, &self.state.title);

        out = casette::render(out, self.x+2, self.y);

        write!(out, "{}\"{}{}\"", cursor::Goto(self.x+7, self.y+5), self.state.title_val, self.state.letter as char).unwrap();
        write!(out, "{} ▲ Letter ▼  ◀ Space ▶", cursor::Goto(self.x+7, self.y+16)).unwrap();

        write!(out, "{}{}{}  clear  ", cursor::Goto(self.x+24, self.y+18), color::Bg(color::Yellow), color::Fg(color::Black)).unwrap();
        write!(out, "{}{}{}  .xml  ", cursor::Goto(self.x+24, self.y+20), color::Bg(color::Green), color::Fg(color::Black)).unwrap();

        out = button::render(out, self.x+2, self.y+18, 20, "Create", Color::Red, true);

        write!(out, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());

        match action {
            Action::SelectR => Action::CreateProject(self.state.title_val.clone()),
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
