use termion::raw::{RawTerminal};
use termion::{cursor};

use std::io::{Write, Stdout};

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16, 
        width: u16, title: &str) -> RawTerminal<Stdout> {
    let title_len = title.len() as u16;
    for i in 0..width {
    for j in 0..3 {
        let left = i == 0;
        let top = j == 0;
        let right = i == width-1;
        let bottom = j == 2;
        write!(out, "{}{}",
            cursor::Goto(x+i, y+j),
            match (top, right, bottom, left){
                // TOP LEFT
                (true, false, false, true) => "┌",
                (false, true, true, false) => "┘",
                (true, true, false, false) => "┐",
                (false, false, true, true) => "└",
                (false, false, false, true) => "│",
                (false, true, false, false) => "│",
                (true, false, false, false) => "─",
                (false, false, true, false) => "─",
                _ => " "
            }).unwrap();
    }}
    let title_x = x + (width/2) - (title_len/2);
    write!(out, "{}{}", cursor::Goto(title_x, y+1), title).unwrap();
    out
}
