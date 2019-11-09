use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

pub fn render(mut out: RawTerminal<Stdout>, 
    origin_x: u16, 
    origin_y: u16, 
    width: u16, 
    height: u16,
    title: &String
) -> RawTerminal<Stdout> {
    for x in 0..width {
	for y in 0..height {
	    let left = x == 0;
	    let top = y == 0;
	    let right = x == width-1;
	    let bottom = y == height-1;
	    write!(out, "{}{}{}{}",
		cursor::Goto(origin_x+x, origin_y+y),
		color::Fg(color::Black),
		color::Bg(color::LightYellow),
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
	    if right || bottom {
			write!(out, "{}{}  ",
				cursor::Goto(origin_x+x+1, origin_y+y+1),
				color::Bg(color::LightBlue)).unwrap();
	    }
	    let title_len = title.len() as u16;
	    let title_x = (width/2) - (title_len/2);
	    write!(out, "{}{}{} {} ",
		cursor::Goto(origin_x+title_x, origin_y),
		color::Bg(color::LightYellow),
		color::Fg(color::Black),
		title).unwrap();
	}
    }
    out
}
