use termion::{color, cursor};
use std::io::{Write, Stdout};
use crate::common::{Screen, Action, Window, Note};
use crate::common::{char_offset};
use itertools::Itertools;

const C_POSITION: u16 = 13;

pub fn render(out: &mut Screen, 
    window: Window,
    start: u16,
    sample_rate: u32,
    bpm: u16,
    zoom: usize,
    notes: &Vec<Note>) {

    let mut sorted_notes: Vec<Note> = notes.iter().map(|n| 
        // Shift up a half step because c shares a row with b
        Note {
            note: n.note + 1,
            id: n.id,
            t_in: n.t_in,
            t_out: n.t_out,
            vel: n.vel,
        }
    ).collect();

    sorted_notes.sort_by(|a, b| a.note.partial_cmp(&b.note).unwrap());

    let mut canvas: Vec<Vec<&str>> = vec![vec![""; window.w as usize]; window.h as usize];

    for note in sorted_notes.iter() {

        let x_in = char_offset(note.t_in, sample_rate, bpm, zoom) - start;
        let x_out = char_offset(note.t_out, sample_rate, bpm, zoom) - start;
        let len = if x_out - x_in > 0 { x_out - x_in } else { 1 }; 

        // Note ends before the window
        if x_out < 0 { continue; }
        // Note begins after the window
        if x_in > window.w { continue; }

        let y_pos = window.y + window.h - (
            (note.note as u16 - 60) / 2 + C_POSITION
        );
        let y_size: usize = if y_pos < 0 { 0 } 
            else if y_pos >= (canvas.len() as u16) { canvas.len() - 1 }
            else { y_pos as usize };

        if note.note % 2 == 0 {
            for j in 0..len {
                if x_in + j < window.w {
                    canvas[y_size][(x_in + j) as usize] = "▄";
                }
            } 
        } else {
            for j in 0..len {
                if x_in + j < window.w {
                    if canvas[y_size][(x_in + j) as usize] == "█" { 
                        continue;
                    } 
                    canvas[y_size][(x_in + j) as usize] = 
                        if canvas[y_size][(x_in + j) as usize] == "▄" { "█" } 
                        else { "▀" }
                }
            };
        };
    }

    for (i, line) in canvas.iter().enumerate() {
        for (j, beat) in line.iter().enumerate() {
            if *beat != "" {
                write!(out, "{}{}", cursor::Goto(
                    window.x + (j as u16),
                    window.y+i as u16,
                ), beat).unwrap();
            }
        }
    }
}
