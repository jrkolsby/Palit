use termion::raw::{RawTerminal};
use termion::{cursor};

use std::io::{Write, Stdout};

use crate::common::{Window};
use crate::components::border;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16, 
        width: u16, title: &str) -> RawTerminal<Stdout> {
    let title_len = title.len() as u16;
    out = border::render(out, Window {
        x,
        y,
        w: width,
        h: 3,
    });
    let title_x = x + (width/2) - (title_len/2);
    write!(out, "{}{}", cursor::Goto(title_x, y+1), title).unwrap();
    out
}
