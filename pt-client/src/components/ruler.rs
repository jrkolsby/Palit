use termion::raw::{RawTerminal};
use termion::{cursor};
use std::io::{Write, Stdout};
use crate::common::{Color, write_fg, write_bg};

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
) -> RawTerminal<Stdout>{
    if scroll == 0 {
        write!(out, "{}{{{{", cursor::Goto(origin_x-2, origin_y)).unwrap()
    }
    let mut beat = 0;
    let _zoom = zoom as u16;
    eprintln!("{}, {}", playhead, scroll);
    for i in 0..width {
        if i == playhead-scroll {
            for j in 1..height {
                write!(out, "{}|", cursor::Goto(origin_x+i, origin_y+j));
            }
            out = write_fg(out, Color::Red);
        } else {
            out = write_fg(out, Color::White);
        }
        if i % _zoom == 0 {
            let glyph = if (beat+scroll+1) % time_beat as u16 == 0 { "!" } else { "." };
            write!(out, "{}{}",
                cursor::Goto(origin_x+i, origin_y),
                glyph).unwrap();
            beat += 1;
        }
    }
    out
}
