use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

use crate::common::Window;

pub fn render(mut out: RawTerminal<Stdout>, win: Window) -> RawTerminal<Stdout> {
    for x in 0..win.w {
        for y in 0..win.h {
            let left = x == 0;
            let top = y == 0;
            let right = x == win.w-1;
            let bottom = y == win.h-1;
            write!(out, "{}{}", cursor::Goto(win.x+x, win.y+y),
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
                }
            ).unwrap();
        }
    }
    out
}
