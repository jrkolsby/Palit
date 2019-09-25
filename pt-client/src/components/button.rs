use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

use crate::common::Color;

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
	width: u16,
    title: &str,
	bg_color: Color,
    active: bool,
) -> RawTerminal<Stdout> {
    let title_len = title.len() as u16;
    for x in 0..width {
	for y in 0..3 {
	    let left = x == 0;
	    let top = y == 0;
	    let right = x == width-1;
	    let bottom = y == 2;
	    if active {
			write!(out, "{}", color::Fg(color::Black));
			match bg_color {
				Color::Red => write!(out, "{}", color::Bg(color::Red)),
				Color::Pink => write!(out, "{}", color::Bg(color::Magenta)),
				Color::Blue => write!(out, "{}", color::Bg(color::Blue)),
				Color::Green => write!(out, "{}", color::Bg(color::Green)),
				Color::Yellow => write!(out, "{}", color::Bg(color::Yellow)),
			};
	    } else {
			write!(out, "{}{}",
				color::Fg(color::White),
				color::Bg(color::Reset));
	    }
	    write!(out, "{}{}",
		cursor::Goto(origin_x+x, origin_y+y),
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
	}
    }
    let title_x = origin_x + (width/2) - (title_len/2);
    write!(out, "{}{}", cursor::Goto(title_x, origin_y+1), title);
	write!(out, "{}{}", color::Fg(color::White), color::Bg(color::Reset));
    out
}
