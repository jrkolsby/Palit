use std::io::{Write, Stdout};

use termion::raw::{RawTerminal};
use termion::{color, cursor};

use crate::common::Window;
use crate::components::border;

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
    width: u16, 
    height: u16,
    title: &String
) -> RawTerminal<Stdout> {
    write!(out, "{}{}", color::Bg(color::LightBlue), color::Fg(color::Black)).unwrap();

    for x in 0..width {
        write!(out, "{}{}  ",
            cursor::Goto(origin_x+x+1, origin_y+height),
            color::Bg(color::LightBlue)).unwrap();
    }

    for y in 0..height {
        write!(out, "{}{}  ",
            cursor::Goto(origin_x+width, origin_y+y+1),
            color::Bg(color::LightBlue)).unwrap();
    }

    write!(out, "{}", color::Bg(color::LightYellow)).unwrap();

    out = border::render(out, Window {
        x: origin_x,
        y: origin_y,
        w: width,
        h: height,
    });

    let title_len = title.len() as u16;
    let title_x = (width/2) - (title_len/2);
    write!(out, "{}{}{} {} ",
        cursor::Goto(origin_x+title_x, origin_y),
        color::Bg(color::LightYellow),
        color::Fg(color::Black),
        title).unwrap();
    out
}
