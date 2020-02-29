use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, Color, write_bg, write_fg};

const KEYBOARD: &str = r#"
[][][][][][_________][][][][]
[__][][][][][][][][][][][___]
[_][][][][][][][][][][][][__]
[][][][][][][][][][][][][][_]
[_][][][][][][][][][][][][][]
"#;

pub fn render(out: &mut Screen, current_actions: &Vec<Action>, x: u16, y: u16) {
    for (i, line) in KEYBOARD.lines().enumerate() {
        for (j, key_char) in line.chars().enumerate() {
            let (bg, fg) = match (i,j) {
                (3,4) | // NoteOn C
                (5,4) | // NoteOn D
                (7,4) | // NoteOn E
                (9,4) | // NoteOn F
                (11,4) | // NoteOn G
                (13,4) | // NoteOn A
                (15,4) | // NoteOn B
                (17,4) | // ...
                (19,4) |
                (21,4) |
                (23,4) |
                (23,4) => (Color::White, Color::Black), 
                (4,3) | // NoteOn C#
                (6,3) | // NoteOn D#
                (10,3) | // NoteOn F#
                (12,3) | // NoteOn G#
                (14,3) | // NoteOn A#
                (18,3) | // ...
                (20,3) => (Color::Black, Color::White),
                _ => (Color::Beige, Color::Black),
            };
            write_bg(out, bg);
            write_fg(out, fg);
        }
        write!(out, "{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            line).unwrap();
    };
}
