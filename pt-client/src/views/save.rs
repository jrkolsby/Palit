use std::io::{Write, Stdout};
use termion::{color, cursor};
use xmltree::Element;
use libcommon::Action;

use crate::views::Layer;
use crate::components::{popup, casette, button, bigtext};
use crate::common::{Screen, Color, write_bg, write_fg};

pub struct Save {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    state: SaveState,
}

#[derive(Clone, Debug)]
pub struct SaveState {
    title_str: String,
    last_char: u8,
}

const PADDING: (u16, u16) = (2, 2);
const SIZE: (u16, u16) = (32, 10);
const TEXT_SIZE: (u16, u16) = (50, 3);
const CAPS_MAX: u8 = 90;
const CAPS_MIN: u8 = 65;

const NUM_MAX: u8 = 57;
const NUM_MIN: u8 = 48;

const LOWER_MAX: u8 = 122;
const LOWER_MIN: u8 = 97;

fn reduce(state: SaveState, action: Action) -> SaveState {
    SaveState {
        title_str: match action {
            Action::Right => format!("{}{}", state.title_str, state.last_char as char),
            Action::Left => if state.title_str.len() > 0 {
                state.title_str[..state.title_str.len() - 1].to_string()
            } else {
                state.title_str.clone()
            },
            _ => state.title_str.clone(),
        },
        last_char: match action {
            Action::Up => match state.last_char {
                CAPS_MAX => CAPS_MIN,
                NUM_MAX => NUM_MIN,
                LOWER_MAX => LOWER_MIN,
                n => n + 1,
            },
            Action::Down => match state.last_char {
                CAPS_MIN => CAPS_MAX,
                NUM_MIN => NUM_MAX,
                LOWER_MIN => LOWER_MAX,
                n => n - 1,
            },
            Action::SelectY => CAPS_MIN,
            Action::SelectP => LOWER_MIN,
            Action::SelectG => NUM_MIN,
            _ => state.last_char,
        },
    }
}

impl Save {
    pub fn new(x: u16, y: u16, width: u16, height: u16, initial: String) -> Self {

        // Initialize State
        let initial_state: SaveState = SaveState {
            last_char: initial.chars().last().unwrap() as u8,
            title_str: initial[0..initial.len() - 1].to_string(),
        };

        Save {
            x: x,
            y: y,
            width: width,
            height: height,
            state: initial_state
        }
    }
}

impl Layer for Save {
    fn render(&self, out: &mut Screen, target: bool) {

        popup::render(out, 
            self.x, 
            self.y, 
            self.width, 
            self.height, 
            &"What's it called?".to_string());

        casette::render(out, 
            self.x + if SIZE.0 > self.width { 0 } else { (self.width / 2) - (SIZE.0 / 2) },
            self.y + if SIZE.1 > self.height { 0 } else { (self.height / 2) - (SIZE.1 / 2) });

        bigtext::render(out, 
            self.x + if TEXT_SIZE.0 > self.width { 0 } else { (self.width / 2) - (TEXT_SIZE.0 / 2) }, 
            self.y + if TEXT_SIZE.1 > self.height { 0 } else { (self.height / 2) - (TEXT_SIZE.1 / 2) },
            format!("{}{}_", 
                self.state.title_str, 
                self.state.last_char as char)
            );

        write_fg(out, Color::White);
        write_bg(out, Color::Black);
        write!(out, "{} ▲ Letter ▼ ", cursor::Goto(
            self.x + PADDING.0, 
            self.y + PADDING.1 + 1)).unwrap();

        write!(out, "{} ◀ Space ▶ ", cursor::Goto(
            self.x + PADDING.0, 
            self.y + PADDING.1 + 1)).unwrap();


        write_fg(out, Color::Black);
        write_bg(out, Color::Yellow);
        write!(out, "{} UPPERCASE ", cursor::Goto(
            self.x + PADDING.0, 
            self.y + PADDING.1 + 1)).unwrap();

        write_bg(out, Color::Pink);
        write!(out, "{} lowercase ", cursor::Goto(
            self.x + PADDING.0, 
            self.y + PADDING.1 + 3)).unwrap();

        write_bg(out, Color::Green);
        write!(out, "{} 1234... ", cursor::Goto(
            self.x + PADDING.0, 
            self.y + PADDING.1 + 5)).unwrap();

        write_bg(out, Color::Blue);
        button::render(out, 
            self.x + PADDING.0, 
            self.y + self.height - PADDING.1 - 3, 
            self.width - (2 * PADDING.1), 
            "Save to disk");
    }

    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action.clone());

        match action {
            Action::Back => Action::Cancel,
            Action::SelectB => Action::SaveAs(
                format!("{}{}", self.state.title_str, self.state.last_char as char)),
            _ => Action::Noop
        }
    }
    fn alpha(&self) -> bool { true }
    fn save(&self) -> Option<Element> { None }
}
