use std::fs::File;

use itertools::Itertools;

use cursive::{Cursive, Printer};

use wavefile::WaveFile;

 // (bars, beats)
struct time ( i32, i32 );


const TRACK_WIDTH: usize = 20;
const TRACK_HEIGHT: usize = 4;

// #[derive(Clone, Copy)]
struct Region {
    // buffer: Vec<Vec<i32>>, TODO: Implement buffers
    sound_file: WaveFile,    
    time_in: time,
    time_out: time,
    length: time,
}
//impl Copy for Region {} TODO: Allow duplicating a region

fn time_to_offset(width: i32, l: time, t: time) -> i32 {
    (t.0 / l.0) / width
}

fn file_to_pairs(file: WaveFile) -> Vec<(i32, i32)> {

    let chunk_size = file.len() / TRACK_WIDTH;
    let chunks = &file.iter().chunks(chunk_size);

    let values = chunks.into_iter().map( |chunk| {
        let max = chunk.into_iter().map( |frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(TRACK_WIDTH).collect::<Vec<i32>>();

    let global_max = *values.iter().max().unwrap();
    let scale: f64 = TRACK_HEIGHT as f64 / global_max as f64;

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

pub struct Track {
    muted: bool,
    armed: bool,
    solo: bool,
    regions: Vec<Region>,
    length: time,
    width: i32
}

impl cursive::view::View for Track {

    fn draw(&self, printer: &Printer) {
        for (i, region) in self.regions.iter().enumerate() {
            
        }
    }
}
