use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

use crate::common::Action;

const ASSET: &str = r#"
  ┌────────────────────────┬──┐
  │                        │  |
  │                        │  │
  │                        │  │
  │                        │  │
  │                        │  │
  //       stieny         //  │
 //      /=======/       //   │
|’,,,_,,_,,,_,,_,,,_,,__|’    │
|||||||||||||||||||||||||| ,  │
└────────────────────────┘’|  │
  │                        │  │
  │                        |  │
  │                        │  │
  │                        │  /
  │                        │ / 
  │                        │/   
  └────────────────────────┘   
"#;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16, notes: &Vec<Action>) -> RawTerminal<Stdout> {
    for (i, line) in ASSET.lines().enumerate() {
        write!(out, "{}{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Bg(color::Reset),
            color::Fg(color::White),
            line).unwrap();
    };
    for action in notes.iter() {
        let (dx,dy,len) = match action {
            Action::NoteOn(72, _) => (5,10,1),
            Action::NoteOn(73, _) => (6,9,1),
            Action::NoteOn(74, _) => (7,10,1),
            Action::NoteOn(75, _) => (8,9,1),
            Action::NoteOn(76, _) => (9,10,1),
            Action::NoteOn(77, _) => (10,10,1),
            Action::NoteOn(78, _) => (11,9,1),
            Action::NoteOn(79, _) => (12,10,1),
            Action::NoteOn(80, _) => (13,9,1),
            Action::NoteOn(81, _) => (14,10,1),
            Action::NoteOn(82, _) => (15,9,1),
            Action::NoteOn(83, _) => (16,10,1),
            Action::NoteOn(84, _) => (17,10,1),
            Action::NoteOn(85, _) => (18,9,1),
            Action::NoteOn(86, _) => (19,10,1),
            Action::NoteOn(87, _) => (20,9,1),
            Action::NoteOn(88, _) => (21,10,1),
            Action::NoteOn(89, _) => (22,10,1),
            _ => (0,0,0)
        };
        let line: &str = &ASSET.lines().nth(dy).unwrap();
        let fg: String = line.chars().take(dx+len).skip(dx).collect();
        write!(out, "{}{}{}{}",
            cursor::Goto(x+(dx as u16), 1+y+(dy as u16)),
            color::Fg(color::Black),
            color::Bg(color::Red),
            fg).unwrap();
    }
    out
}
