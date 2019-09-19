use termion::raw::{RawTerminal};
use termion::{color, cursor};
use std::io::prelude::*;

use std::io::{Write, Stdout};

const WIDTH: u16 = 10;
const HEIGHT: u16 = 3;

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
    time_note: usize,
    time_beat: usize,
    duration_measure: usize,
    duration_beat: usize,
    bpm: u16,
    metronome: bool,
    focus: bool,
) -> RawTerminal<Stdout> {
        for mut i in 0..WIDTH+1 as u16 {
            for mut j in 1..HEIGHT+1 as u16 {
                if focus {
                    write!(out, "{}{}{} ",
                        cursor::Goto(origin_x-i, origin_y+j),
                        color::Fg(color::Black),
                        color::Bg(color::Green));
                } else {
                    write!(out, "{}{}{} ",
                        cursor::Goto(origin_x-i, origin_y+j),
                        color::Fg(color::White),
                        color::Bg(color::Black));
                }
                // x and y start at 1
                match (i,j) {
                    (1, 1) => {
                        match (metronome, focus) {
                            (true,true) => write!(out, "{} ", color::Bg(color::Black)),
                            (true,false) => write!(out, "{} ", color::Bg(color::White)),
                            _ => { Ok(()) },
                        };
                    },
                    (3, 1) => {
                        match (metronome, focus) {
                            (false,true) => write!(out, "{} ", color::Bg(color::Black)),
                            (false,false) => write!(out, "{} ", color::Bg(color::White)),
                            _ => { Ok(()) },
                        };
                    },
                    (2, 2) => {
                        match (metronome) {
                            true => write!(out, "/"),
                            false => write!(out, "\\"),
                        };
                    }
                    (7, 1) => { write!(out, "{}", bpm); }
                    (7, 2) => { write!(out, "BPM"); }
                    (10, 1) => { write!(out, "{}", time_beat); }
                    (10, 2) => { write!(out, "{}", time_note); }
                    (2, 3) => { write!(out, "{:02}", duration_beat); }
                    (5, 3) => { write!(out, "{}:", duration_measure); }
                    _ => {}
                }
            }
        }
    // Clean up after ourselves
    write!(out, "{}{}",
        color::Fg(color::Reset),
        color::Bg(color::Reset));
    out
}