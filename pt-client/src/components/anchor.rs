use std::io::{Write, Stdout};

use termion::raw::{RawTerminal};
use termion::{color, cursor};

use crate::common::{Anchor, Window};

pub fn render(mut out: RawTerminal<Stdout>, win: Window, a: Anchor) -> RawTerminal<Stdout> {
    for i in 0..a.x {
        write!(out, "{}─", cursor::Goto(win.x+i, a.y));
    }
    out
}