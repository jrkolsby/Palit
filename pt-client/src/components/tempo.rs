use termion::{cursor};
use std::io::{Write, Stdout};
use crate::common::{Screen, Color, write_bg, write_fg};

const WIDTH: u16 = 10;
const HEIGHT: u16 = 3;

pub fn render(out: &mut Screen, 
    origin_x: u16, 
    origin_y: u16, 
    time_note: usize,
    time_beat: usize,
    bpm: u16,
    metronome: bool) {
    for i in 0..WIDTH+1 as u16 {
        for j in 1..HEIGHT+1 as u16 {
            write!(out, "{} ",
                cursor::Goto(origin_x-i, origin_y+j)).unwrap();
            match (i,j) {
                (1, 1) => { if (metronome) { write!(out, "@").unwrap(); } },
                (3, 1) => { if (!metronome) { write!(out, "@").unwrap(); } },
                (2, 2) => {
                    match metronome {
                        true => write!(out, "/").unwrap(),
                        false => write!(out, "\\").unwrap(),
                    };
                }
                (7, 1) => { write!(out, "{}", bpm).unwrap(); }
                (7, 2) => { write!(out, "BPM").unwrap(); }
                (10, 1) => { write!(out, "{}", time_beat).unwrap(); }
                (10, 2) => { write!(out, "{}", time_note).unwrap(); }
                _ => {}
            }
        }
    }
}
