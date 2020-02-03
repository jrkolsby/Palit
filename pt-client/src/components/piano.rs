use std::io::Write;
use termion::{color, cursor};
use libcommon::Action;

use crate::common::Screen;

const ASSET: &str = r#"
  ┌─────────────────────────────────────────────────────┬──┐
  │                                                     │  |
  │                                                     │  │
  │                                                     │  │
  │                                                     │  │
  │                                                     │  │
  //                     hammond                       //  │
 //                     /=======/                     //   │
|’_,,,_,,_,,,_,,_,,,_,,_,,,_,,_,,,_,,_,,,_,,_,,,_,,__|’    │
||||||||||||||||||||||||||||||||||||||||||||||||||||||| ,  │
└─────────────────────────────────────────────────────┘’|  │
  │                                                     │  │
  │                                                     |  │
  │                                                     │  │
  │                                                     │  /
  │                                                     │ / 
  │                                                     │/   
  └─────────────────────────────────────────────────────┘   
"#;

pub fn render(out: &mut Screen, x: u16, y: u16, notes: &Vec<Action>) {
    for (i, line) in ASSET.lines().enumerate() {
        write!(out, "{}{}{}{}",
            cursor::Goto(x, (i as u16)+y+1),
            color::Bg(color::Reset),
            color::Fg(color::White),
            line).unwrap();
    };
    for action in notes.iter() {
        let (dx,dy,len) = match action {
            Action::NoteOn(24, _) => (6,10,1),
            Action::NoteOn(25, _) => (7,9,1),
            Action::NoteOn(26, _) => (7,10,1),
            Action::NoteOn(27, _) => (8,9,2),
            Action::NoteOn(28, _) => (8,10,1),
            Action::NoteOn(29, _) => (9,10,1),
            Action::NoteOn(30, _) => (10,9,1),
            Action::NoteOn(31, _) => (10,10,1),
            Action::NoteOn(32, _) => (11,9,1),
            Action::NoteOn(33, _) => (11,10,1),
            Action::NoteOn(34, _) => (12,9,2),
            Action::NoteOn(35, _) => (12,10,1),

            Action::NoteOn(36, _) => (13,10,1),
            Action::NoteOn(37, _) => (14,9,1),
            Action::NoteOn(38, _) => (14,10,1),
            Action::NoteOn(39, _) => (15,9,2),
            Action::NoteOn(40, _) => (15,10,1),
            Action::NoteOn(41, _) => (16,10,1),
            Action::NoteOn(42, _) => (17,9,1),
            Action::NoteOn(43, _) => (17,10,1),
            Action::NoteOn(44, _) => (18,9,1),
            Action::NoteOn(45, _) => (18,10,1),
            Action::NoteOn(46, _) => (19,9,2),
            Action::NoteOn(47, _) => (19,10,1),

            Action::NoteOn(48, _) => (20,10,1),
            Action::NoteOn(49, _) => (21,9,1),
            Action::NoteOn(50, _) => (21,10,1),
            Action::NoteOn(51, _) => (22,9,2),
            Action::NoteOn(52, _) => (22,10,1),
            Action::NoteOn(53, _) => (23,10,1),
            Action::NoteOn(54, _) => (24,9,1),
            Action::NoteOn(55, _) => (24,10,1),
            Action::NoteOn(56, _) => (25,9,1),
            Action::NoteOn(57, _) => (25,10,1),
            Action::NoteOn(58, _) => (26,9,2),
            Action::NoteOn(59, _) => (26,10,1),

            Action::NoteOn(60, _) => (27,10,1),
            Action::NoteOn(61, _) => (28,9,1),
            Action::NoteOn(62, _) => (28,10,1),
            Action::NoteOn(63, _) => (29,9,2),
            Action::NoteOn(64, _) => (29,10,1),
            Action::NoteOn(65, _) => (30,10,1),
            Action::NoteOn(66, _) => (31,9,1),
            Action::NoteOn(67, _) => (31,10,1),
            Action::NoteOn(68, _) => (32,9,1),
            Action::NoteOn(69, _) => (32,10,1),
            Action::NoteOn(70, _) => (33,9,2),
            Action::NoteOn(71, _) => (33,10,1),

            Action::NoteOn(72, _) => (34,10,1),
            Action::NoteOn(73, _) => (35,9,1),
            Action::NoteOn(74, _) => (35,10,1),
            Action::NoteOn(75, _) => (36,9,2),
            Action::NoteOn(76, _) => (36,10,1),
            Action::NoteOn(77, _) => (37,10,1),
            Action::NoteOn(78, _) => (37,9,1),
            Action::NoteOn(79, _) => (38,10,1),
            Action::NoteOn(80, _) => (49,9,1),
            Action::NoteOn(81, _) => (49,10,1),
            Action::NoteOn(82, _) => (40,9,2),
            Action::NoteOn(83, _) => (40,10,1),

            Action::NoteOn(84, _) => (41,10,1),
            Action::NoteOn(85, _) => (42,9,1),
            Action::NoteOn(86, _) => (42,10,1),
            Action::NoteOn(87, _) => (43,9,2),
            Action::NoteOn(88, _) => (43,10,1),
            Action::NoteOn(89, _) => (44,10,1),
            Action::NoteOn(90, _) => (44,9,1),
            Action::NoteOn(91, _) => (45,10,1),
            Action::NoteOn(92, _) => (46,9,1),
            Action::NoteOn(93, _) => (46,10,1),
            Action::NoteOn(94, _) => (47,9,2),
            Action::NoteOn(95, _) => (47,10,1),

            Action::NoteOn(96, _) => (48,10,1),
            Action::NoteOn(97, _) => (49,9,1),
            Action::NoteOn(98, _) => (49,10,1),
            Action::NoteOn(99, _) => (50,9,2),
            Action::NoteOn(100, _) => (50,10,1),
            Action::NoteOn(101, _) => (51,10,1),
            Action::NoteOn(102, _) => (51,9,1),
            Action::NoteOn(103, _) => (52,10,1),
            Action::NoteOn(104, _) => (53,9,1),
            Action::NoteOn(105, _) => (53,10,1),
            Action::NoteOn(106, _) => (54,9,2),
            Action::NoteOn(107, _) => (54,10,1),

            _ => (0,0,0)
        };
        let line: &str = &ASSET.lines().nth(dy).unwrap();
        let fg: String = line.chars().take(dx+len).skip(dx).collect();
        write!(out, "{}{}{}{}",
            cursor::Goto(x+(dx as u16), 1+y+(dy as u16)),
            color::Fg(color::Black),
            color::Bg(color::Red),
            fg).unwrap();
    }
}
