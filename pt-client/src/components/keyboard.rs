use termion::raw::{RawTerminal};
use termion::{color, cursor};
use std::io::prelude::*;

use std::io::{Write, Stdout};

const KEYBOARD: &str = r#"
[][][][][][_________][][][][]
[__][][][][][][][][][][][___]
[_][][][][][][][][][][][][__]
[][][][][][][][][][][][][][_]
[_][][][][][][][][][][][][][]
"#;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16) -> RawTerminal<Stdout> {
    for (i, line) in KEYBOARD.lines().enumerate() {
        write!(out, "{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Fg(color::Black),
            line).unwrap();
    };
    out
}