use cursive::{Printer};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::event::{Event, EventResult};
use cursive::direction::Direction;
use cursive::vec::Vec2;

use itertools::Itertools;

use wavefile::{WaveFile,WaveError};

const WAVEFORM_HEIGHT: usize = 4;
const WAVEFORM_WIDTH: usize = 200;

//use core::Buffer;

// TODO Vec::with_capacity()

fn file_to_pairs(file: WaveFile) -> Vec<(i32, i32)> {

    let chunk_size = file.len() / WAVEFORM_WIDTH;
    let chunks = &file.iter().chunks(chunk_size);

    let values = chunks.into_iter().map( |chunk| {
        let max = chunk.into_iter().map( |frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(WAVEFORM_WIDTH).collect::<Vec<i32>>();

    let global_max = *values.iter().max().unwrap();
    let scale: f64 = WAVEFORM_HEIGHT as f64 / global_max as f64;

    //println!("GLOBAL_MAX: {}", global_max);
    //println!("SCALE: {}", scale);

    let mut pairs = vec![];
    for (i, value) in values.iter().enumerate() {
        if i % 2 > 0 {
            continue;
        }
        //println!("{:?} , {:?}", *value, values[i+1]);
        let tick: (i32, i32) = (((*value as f64) * scale).round() as i32, 
                                ((values[i+1] as f64) * scale).round() as i32);

        pairs.push(tick);
    }

    pairs
}

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

pub struct Waveform {
    //sound_file: File,
    //buffer: &'a Buffer,
    color: Color,
    chars: Vec<(i32, i32)>, 
}

impl Waveform {
    pub fn new(color: Color) -> Self {

        let default_file: WaveFile = match WaveFile::open("examples/test.wav") {
            Ok(f)  => f,
            Err(e) => panic!("{}",  e)
        };
        
        Waveform {
            color: color,
            chars: file_to_pairs(default_file)
        }
    }
}

impl cursive::view::View for Waveform {

    fn draw(&self, printer: &Printer) {
        for (i, pair) in self.chars.iter().enumerate() {
            let text = pair_to_char(*pair);
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::White), {
                    match printer.focused {
                        true => Color::Dark(BaseColor::Red),
                        _ => self.color
                    }

                }),
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
        Vec2::new(self.chars.len(), 3)
    }    
}
