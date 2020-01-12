use crate::common::Screen;
use termion::cursor;
use std::io::Write;

pub fn pair_to_char(pair: (u8, u8)) -> char {

    // MAX IS 65,536
    let a: usize = (pair.0 >= 1) as usize;
    let b: usize = (pair.0 >= 2) as usize;
    let c: usize = (pair.0 >= 3) as usize;
    let d: usize = (pair.0 >= 4) as usize;
    let e: usize = (pair.1 >= 1) as usize;
    let f: usize = (pair.1 >= 2) as usize;
    let g: usize = (pair.1 >= 3) as usize;
    let h: usize = (pair.1 >= 4) as usize;

    braille::BRAILLE[d][h]
                    [c][g]
                    [b][f]
                    [a][e]
}

pub fn render(out: &mut Screen, pairs: &[(u8, u8)], x: u16, y: u16) {
    for (i, pair) in pairs.iter().enumerate() {
        write!(out, "{}{:}",
            cursor::Goto(x+(i as u16),y),
            pair_to_char(*pair)).unwrap();
    }
}
