use std::fs::File;

use cursive::{Cursive, Printer};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::event::{Event, EventResult};
use cursive::direction::Direction;
use cursive::vec::Vec2;

//use core::Buffer;

// TODO Vec::with_capacity()

fn pair_to_char(pair: (i32, i32)) -> char {

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

struct Waveform {
    //sound_file: File,
    //buffer: &'a Buffer,
    color: Color,
    chars: Vec<(i32, i32)>, 
}

impl cursive::view::View for Waveform {
    fn draw(&self, printer: &Printer) {
        for (i, pair) in self.chars.iter().enumerate() {
            let text = pair_to_char(*pair);
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::White), self.color),
                |printer| printer.print((i, 0), &text.to_string()),
            )
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(self.chars.len(), 1)
    }    
}
