use termion::{color, cursor};
use std::io::{Write, Stdout};
use crate::common::{Screen, Action, Window};
use itertools::Itertools;

const ASSET: &str = r#"
▄▄██
▄▄██
▄▄██
▀▀██
▀▀██
████
"#;

const C_POSITION: u16 = 13;

pub fn render(out: &mut Screen, 
    window: Window,
    octaves: u16, 
    active: &Vec<Action>) {

    for i in 1..octaves {
        for (j, line) in ASSET.lines().enumerate() {
            write!(out, "{}{}", cursor::Goto(
                window.x, window.y + window.h - 1 - (i * 6) + (j as u16) 
            ), line).unwrap();
        };
    }

    let mut sorted_notes: Vec<u16> = active.iter().map(|a| 
        // Shift up a half step because c shares a row with b
        match a { Action::NoteOn(a, _) => *a + 1, _ => 0}
    ).unique().collect();

    sorted_notes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for i in 0..sorted_notes.len() {
        let key = sorted_notes[i]; 

        let glyph: String = if key % 2 == 0 {
            (0..window.w-5).map(|_| "▄").collect::<String>()
        } else {
            if i > 0 && sorted_notes[i-1] == key-1 {
                (0..window.w-5).map(|_| "█").collect::<String>()
            } else {
                (0..window.w-5).map(|_| "▀").collect::<String>()
            }
        };
        let y_pos = window.y + window.h - (
            (key / 2) - 30 + C_POSITION
        );

        write!(out, "{}{}", cursor::Goto(
            window.x + 5,
            y_pos
        ), glyph).unwrap();
    }
}
