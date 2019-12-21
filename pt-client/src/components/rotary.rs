use termion::{cursor};
use std::io::prelude::*;
use std::io::{Write, Stdout};
use crate::common::Screen;

pub fn angle_to_char(angle: f32, mirror: bool) -> char {

    // e, f, b, c, g, h, 

    braille::BRAILLE[d][h]
                    [c][g]
                    [b][f]
                    [a][e]
}

pub fn render(out: &mut Screen, x: u16, y: u16, 
        pairs: &Vec<(i32, i32)>) {
    let quadrant: usize = match angle {
        0.0..0.25 => 0
        0.25..0.5 => 1
        0.5..0.75 => 2
        0.75..1.0 => 3
        _ => panic!("Angle overflow"),

    }
    for q in 0..4 {
        write!(out, "{}{:}",
            cursor::Goto(x,y),
            angle_to_char(*pair)).unwrap();
    }
}