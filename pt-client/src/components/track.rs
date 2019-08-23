use itertools::Itertools;

use cursive::Printer;

use wavefile::WaveFile;

 // (bars, beats)
struct Time ( i32, i32 );


const TRACK_WIDTH: usize = 20;
const TRACK_HEIGHT: usize = 4;

// #[derive(Clone, Copy)]
struct Region {
    // buffer: Vec<Vec<i32>>, TODO: Implement buffers
    sound_file: WaveFile,    
    time_in: Time,
    time_out: Time,
    length: Time,
}
//impl Copy for Region {} TODO: Allow duplicating a region

fn time_to_offset(width: i32, l: Time, t: Time) -> i32 {
    (t.0 / l.0) / width
}

pub struct Track {
    muted: bool,
    armed: bool,
    solo: bool,
    regions: Vec<Region>,
    length: Time,
    width: i32
}

impl cursive::view::View for Track {

    fn draw(&self, printer: &Printer) {
        for (i, region) in self.regions.iter().enumerate() {
            
        }
    }
}
