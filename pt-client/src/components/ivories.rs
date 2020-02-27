use std::io::Write;
use termion::cursor;
use itertools::Itertools;
use libcommon::{Action, Key};

use crate::common::{Screen, Window};

const ASSET: &str = r#"
▄▄██
▄▄██
▄▄██
▀▀██
▀▀██
████
"#;

const C_POSITION: i16 = 13;

pub fn render(out: &mut Screen, 
    window: Window,
    octaves: u16, 
    active: &Vec<Action>) {

    for i in 1..octaves {
        for (j, line) in ASSET.lines().enumerate() {
            write!(out, "{}{}", cursor::Goto(
                window.x, 
                ((window.y + window.h) as i16 - 1 - (i as i16 * 6) + (j as i16)) as u16
            ), line).unwrap();
        };
    }

    let mut sorted_notes: Vec<Key> = active.iter().map(|a| 
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
        let y_pos_signed: i16 = (window.y + window.h) as i16 - (
            (key as i16 / 2) - 30 + C_POSITION
        );
        let y_pos = if y_pos_signed > 0 { y_pos_signed as u16 } else { 0 };

        write!(out, "{}{}", cursor::Goto(
            window.x + 5,
            y_pos
        ), glyph).unwrap();
    }
}
