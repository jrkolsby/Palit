use termion::cursor;
use std::io::Write;
use crate::common::{Window, Screen};
use crate::components::border;

pub fn render(out: &mut Screen, x: u16, y: u16, 
        width: u16, title: &str) {
    let title_len = title.len() as u16;
    border::render(out, Window {
        x,
        y,
        w: width,
        h: 3,
    });
    let title_x = x + (width/2) - (title_len/2);
    write!(out, "{}{}", cursor::Goto(title_x, y+1), title).unwrap();
}
