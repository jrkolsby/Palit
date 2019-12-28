use termion::{color, cursor};
use std::io::{Write, Stdout};
use crate::common::{Screen, Action, Window};

const ASSET: &str = r#"
████
▄▄██
▄▄██
▄▄██
▀▀██
▀▀██
"#;

pub fn render(out: &mut Screen, 
    window: Window,
    octaves: u16, 
    active: &Vec<Action>) {
    for i in 0..octaves {
        for (j, line) in ASSET.lines().enumerate() {
            write!(out, "{}{}", cursor::Goto(
                window.x, window.y + (j as u16) + (i * 6) - 1
            ), line).unwrap();
        };
    }
    let note_top = (0..window.w-5).map(|_| "▀").collect::<String>();
    let note_bottom = (0..window.w-5).map(|_| "▄").collect::<String>();
    let note_both = (0..window.w-5).map(|_| "█").collect::<String>();

    for note in active.iter() {
        match note {
            Action::NoteOn(key, v) => write!(out, "{}{}", cursor::Goto(
                window.x + 5,
                window.y + (key / 2) + (key % 2)
            ), note_top).unwrap(),
            _ => {}
        }
    }
}
