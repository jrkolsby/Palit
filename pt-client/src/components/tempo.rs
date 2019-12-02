use termion::raw::{RawTerminal};
use termion::{cursor};
use std::io::{Write, Stdout};
use crate::common::{Color, write_bg, write_fg};

const WIDTH: u16 = 10;
const HEIGHT: u16 = 3;

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
    time_note: usize,
    time_beat: usize,
    bpm: u16,
    metronome: bool,
) -> RawTerminal<Stdout> {
        for i in 0..WIDTH+1 as u16 {
            for j in 1..HEIGHT+1 as u16 {
                write!(out, "{} ",
                    cursor::Goto(origin_x-i, origin_y+j));
                match (i,j) {
                    (1, 1) => { if (metronome) { write!(out, "@"); } },
                    (3, 1) => { if (!metronome) { write!(out, "@"); } },
                    (2, 2) => {
                        match metronome {
                            true => write!(out, "/"),
                            false => write!(out, "\\"),
                        };
                    }
                    (7, 1) => { write!(out, "{}", bpm); }
                    (7, 2) => { write!(out, "BPM"); }
                    (10, 1) => { write!(out, "{}", time_beat); }
                    (10, 2) => { write!(out, "{}", time_note); }
                    _ => {}
                }
            }
        }
    out
}
