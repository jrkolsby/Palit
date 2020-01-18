use termion::cursor;
use std::io::Write;
use crate::common::Screen;

const CASETTE: &str = r#"
  _____________________________ 
 /|............................|
| |:         username         :|
| |:                          :|
| |:     ,-.   _____   ,-.    :|
| |:    ( `)) [_____] ( `))   :|
|v|:     `-`   ' ' '   `-`    :|
|||:     ,______________.     :|
|||...../::::o::::::o::::\.....|
|^|..../:::O::::::::::O:::\....|
|/`---/--------------------`---|
`.___/ /====/ /=//=/ /====/____/
     `--------------------'     
"#;

pub fn render(out: &mut Screen, x: u16, y: u16) {
    for (i, line) in CASETTE.lines().enumerate() {
        write!(out, "{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            line).unwrap();
    };
}
