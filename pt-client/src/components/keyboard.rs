use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::Screen;

const KEYBOARD: &str = r#"
[][][][][][_________][][][][]
[__][][][][][][][][][][][___]
[_][][][][][][][][][][][][__]
[][][][][][][][][][][][][][_]
[_][][][][][][][][][][][][][]
"#;

pub fn render(out: &mut Screen, current_actions: &Vec<Action>, x: u16, y: u16) {
    for (i, line) in KEYBOARD.lines().enumerate() {
        write!(out, "{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            line).unwrap();
    };
    for action in current_actions.iter() {
        let (dx,dy,len) = match action {
            Action::NoteOn(24, _) => (3,4,2),
            Action::NoteOn(25, _) => (4,3,2),
            Action::NoteOn(26, _) => (5,4,2),
            Action::NoteOn(27, _) => (6,3,2),
            Action::NoteOn(28, _) => (7,4,2),
            Action::NoteOn(29, _) => (9,4,2),
            Action::NoteOn(30, _) => (10,3,2),
            Action::NoteOn(31, _) => (11,4,2),
            Action::NoteOn(32, _) => (12,3,2),
            Action::NoteOn(33, _) => (13,4,2),
            Action::NoteOn(34, _) => (14,3,2),
            Action::NoteOn(35, _) => (15,4,2),
            Action::NoteOn(36, _) => (17,4,2),
            Action::NoteOn(37, _) => (18,3,2),
            Action::NoteOn(38, _) => (19,4,2),
            Action::NoteOn(39, _) => (20,3,2),
            Action::NoteOn(40, _) => (21,4,2),
            Action::NoteOn(41, _) => (23,4,2),
            _ => (0,0,0)
        };
        let fg: &str = &KEYBOARD.lines().nth(dy+1).unwrap()[dx..dx+len];
        write!(out, "{}{}",
            cursor::Goto(x+(dx as u16), y+(dy as u16)),
            fg).unwrap();
    }
}
