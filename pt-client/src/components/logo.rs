use termion::raw::{RawTerminal};
use termion::{color, cursor};
use std::io::prelude::*;

use std::io::{Write, Stdout};

const LOGO: &str = r#"
d8888b.  .d8b.  db      d888888b d888888b
88  `8D d8' `8b 88      `~~88~'  `~~88~~'
88oodD' 88ooo88 88         88       88   
88~~~   88~~~88 88         88       88   
88      88   88 88booo.   .88.      88   
YP      YP   YP Y88888P Y888888P    YP   
"#;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16) -> RawTerminal<Stdout> {
    for (i, line) in LOGO.lines().enumerate() {
        write!(out, "{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Fg(color::White),
            line).unwrap();
    };
    out
}