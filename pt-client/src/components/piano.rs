use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

const ASSET: &str = r#"
  ┌────────────────────────┬──┐
  │                        │  |
  │                        │  │
  │                        │  │
  │                        │  │
  │                        │  │
  //       stieny         //  │
 //      /=======/       //   │
‘’,,,_,,_,,,_,,_,,,_,,__’’    │
|||||||||||||||||||||||||| ,  │
└────────────────────────┘’|  │
  │                        │  │
  │                        |  │
  │                        │  │
  │                        │  /
  │                        │ / 
  │                        │/   
  └────────────────────────┘   
"#;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16) -> RawTerminal<Stdout> {
    for (i, line) in ASSET.lines().enumerate() {
        write!(out, "{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Fg(color::Black),
            line).unwrap();
    };
    out
}
