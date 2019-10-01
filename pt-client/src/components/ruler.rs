use termion::raw::{RawTerminal};
use termion::{color, cursor};
use std::io::prelude::*;

use std::io::{Write, Stdout};

const WIDTH: u16 = 10;
const HEIGHT: u16 = 3;

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
    width: u16,
    height: u16,
    time_beat: usize,
    zoom: usize,
    scroll: u16, 
    playhead: u16,
) -> RawTerminal<Stdout> {
    if scroll == 0 {
        write!(out, "{}{{{{", cursor::Goto(origin_x-2, origin_y)).unwrap()
    }
    for i in 0..width {
        let beat = if (i+scroll+1) % time_beat as u16 == 0 { "!" } else { "." };
        let space = {
            let mut a: String = " ".to_string();
            for i in 0..zoom {
                a = format!("{}{}", a, a).to_string();
            }
            a
        };
        if i+scroll == playhead {
            for j in 0..height {
                write!(out, "{}|", cursor::Goto(origin_x+i, origin_y+j));
            }
            write!(out, "{}", color::Fg(color::Red));
        } else {
            write!(out, "{}", color::Fg(color::White));
        }
        write!(out, "{}{}",
            cursor::Goto(origin_x+i, origin_y),
            beat).unwrap()
    }
    out
}