use termion::cursor;
use std::io::Write;
use crate::common::Screen;

pub fn render(out: &mut Screen, 
    origin_x: u16, 
    origin_y: u16, 
    width: u16,
    height: u16,
    meter_beat: u16,
    zoom: usize,
    scroll: u16, 
    playhead: u16) {
    if scroll == 0 {
        write!(out, "{}{{{{", cursor::Goto(origin_x-2, origin_y)).unwrap()
    }
    let mut beat = 0;
    let _zoom = zoom as u16;
    for i in 0..width {
        if i as i16 == playhead as i16 - scroll as i16 {
            for j in 1..height {
                write!(out, "{}|", cursor::Goto(origin_x+i, origin_y+j)).unwrap();
            }
        }
        if i % _zoom == 0 {
            let glyph = if (beat+scroll+1) % meter_beat as u16 == 0 { "!" } else { "." };
            write!(out, "{}{}",
                cursor::Goto(origin_x+i, origin_y),
                glyph).unwrap();
            beat += 1;
        }
    }
}
