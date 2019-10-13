use termion::raw::{RawTerminal};
use termion::{color, cursor};

use std::io::{Write, Stdout};

const SPRITE0: &str = r#"
 ((((---___\/ .
  \    .   \\  
 __O_____O__|\ 
 \_//  \_/   \\
   /        .\\
  /         /\\
 (o O)     /\\\
   |___   /  \ 
    \____/     
"#;

const SPRITE1: &str = r#"
 ((((---___\/  
  \     .  \\ .
 __O_____O__|\ 
 \_//  \_/   \\
   /        .\\
  /         /\\
 (o O)     /\\\
   |___   /  \ 
    \____/     
"#;

const SPRITE2: &str = r#"
 . \/___---())))
   //   .    /   
  /|__O_____O__ 
 //   \_/  \\_/ 
 //.        \   
 //\         \  
 ///\     (O o) 
  /  \   ___|   
      \____/     
"#;

const SPRITE3: &str = r#"
   \/___---))))
 . //    .   /   
  /|__O_____O__ 
 //   \_/  \\_/ 
 //.        \   
 //\         \  
 ///\     (O o) 
  /  \   ___|   
      \____/     
"#;

pub fn render(mut out: RawTerminal<Stdout>, x: u16, y: u16, frame: u16) -> RawTerminal<Stdout> {
    let frame_str = match frame {
        0 => SPRITE0,
        1 => SPRITE1,
        2 => SPRITE2,
        3 => SPRITE3,
    };
    for (i, line) in frame_str.lines().enumerate() {
        write!(out, "{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Fg(color::LightWhite),
            line).unwrap();
    };
    out
}