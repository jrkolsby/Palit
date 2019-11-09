use termion::raw::{RawTerminal};
use termion::{cursor};

use std::io::{Write, Stdout};

use crate::common::{Direction};

pub fn render(mut out: RawTerminal<Stdout>, 
    x: u16, 
    y: u16, 
    title: String, 
    mut len: i16, 
    dir: Direction) -> RawTerminal<Stdout> 
{    
    write!(out, "{}{}", cursor::Goto(x, y+1), title).unwrap();
    let _x: i16 = x as i16;
    let _y: i16 = y as i16;
    let mut dx: i16 = 0;
    let mut dy: i16 = 0;
    let ticks_per_char = match dir {
        Direction::East | Direction::West => 2,
        _ => 4,
    };
    let glyph = match dir {
        Direction::North |
        Direction::South => 
            braille::BRAILLE[0][1]
                            [0][1]
                            [0][1]
                            [0][1],
        Direction::East |
        Direction::West => 
            braille::BRAILLE[0][0]
                            [0][0]
                            [1][1]
                            [0][0],
        Direction::SW |
        Direction::NE => 
            braille::BRAILLE[0][1]
                            [0][1]
                            [1][0]
                            [1][0],
        Direction::NW |
        Direction::SE => 
            braille::BRAILLE[1][0]
                            [1][0]
                            [0][1]
                            [0][1],
    };
    while len >= ticks_per_char {
        write!(out, "{}{} ",
            cursor::Goto((_x+dx) as u16, (_y+dy) as u16),
            glyph).unwrap();
        dx = match dir {
            Direction::North |
            Direction::South => dx,
            Direction::NE |
            Direction::SE |
            Direction::East => dx+1,
            Direction::SW |
            Direction::NW |
            Direction::West => dx-1,
        };
        dy = match dir {
            Direction::East |
            Direction::West => dy,
            Direction::SE |
            Direction::SW |
            Direction::South => dy+1,
            Direction::NE |
            Direction::NW |
            Direction::North => dy-1,
        };
        len = len - ticks_per_char;
    }
    let l0 = (len>0) as usize;
    let l1 = (len>1) as usize;
    let l2 = (len>2) as usize;
    let l3 = (len>3) as usize;
    let over_glyph = match dir {
        Direction::North =>
            braille::BRAILLE[0][l3]
                            [0][l2]
                            [0][l1]
                            [0][l0],
        Direction::South => 
            braille::BRAILLE[0][l0]
                            [0][l1]
                            [0][l2]
                            [0][l3],
        Direction::East => 
            braille::BRAILLE[0][0]
                            [0][0]
                            [l0][l1]
                            [0][0],
        Direction::West => 
            braille::BRAILLE[0][0]
                            [0][0]
                            [l1][l0]
                            [0][0],
        Direction::SW =>
            braille::BRAILLE[0][l0]
                            [0][l1]
                            [l2][0]
                            [l3][0],
        Direction::NE => 
            braille::BRAILLE[0][l3]
                            [0][l2]
                            [l1][0]
                            [l0][0],
        Direction::NW =>
            braille::BRAILLE[l3][0]
                            [l2][0]
                            [0][l1]
                            [0][l0],
        Direction::SE => 
            braille::BRAILLE[l0][0]
                            [l1][0]
                            [0][l2]
                            [0][l3],
    };
    write!(out, "{}{}",
        cursor::Goto((_x+dx) as u16, (_y+dy) as u16),
        over_glyph).unwrap();
    out
}
