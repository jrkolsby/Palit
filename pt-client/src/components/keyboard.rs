use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, Color, write_bg, write_fg};

const KEYBOARD: &str = r#"
         [][][][][][][][][][][][][][_] ◀ PROJECT
ROUTES ▶ [][Q][][][][][][][][][][][][] ◀ [STOP][PLAY]
         [_][][][][][][][][][][][][__] ◀ MODULES
    OCTAVE ▶ [][][][][][][][][][] ◀ THIS SCREEN
         [][][][][_________][][][][][]
"#;

pub fn render(out: &mut Screen, current_actions: &Vec<Action>, x: u16, y: u16) {
    for (i, line) in KEYBOARD.lines().enumerate() {
        for (j, key_char) in line.chars().enumerate() {
            let (bg, fg) = match (j ,i) {
                (12..=13,3) | // NoteOn C
                (14..=15,3) | // NoteOn D
                (16..=17,3) | // NoteOn E
                (18..=19,3) | // NoteOn F
                (20..=21,3) | // NoteOn G
                (22..=23,3) | // NoteOn A
                (24..=25,3) | // NoteOn B
                (26..=27,3) | // ...
                (28..=29,3) |
                (30..=31,3) |
                (32..=33,3) |
                (32..=33,3) => (Color::White, Color::Black), 
                (14..=15,2) | // NoteOn C#
                (16..=17,2) | // NoteOn D#
                (20..=21,2) | // NoteOn F#
                (22..=23,2) | // NoteOn G#
                (24..=25,2) | // NoteOn A#
                (28..=29,2) | // ...
                (30..=31,2) => (Color::Black, Color::White),
                (18..=19,2) => (Color::Yellow, Color::White), 
                (26..=27,2) => (Color::Green, Color::White), 
                (19..=20,4) => (Color::Blue, Color::White), 
                (25..=26,4) => (Color::Pink, Color::White), 
                (17..=27,5) => (Color::Red, Color::White), 
                _ => (Color::Beige, Color::Black),
            };
            write_bg(out, bg);
            write_fg(out, fg);
            write!(out, "{}{}",
                cursor::Goto(
                    (j as u16) + x + 1, 
                    (i as u16) + y + 1
                ),
                key_char).unwrap();
        }
    };
}
