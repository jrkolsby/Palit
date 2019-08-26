// extern crate quick-xml;

use std::fs::File;

use cursive::theme::{Color, BaseColor};

pub struct TimelineState {
    pub name: String,
    pub tempo: u16,
    pub time_beat: usize, // TOP 
    pub time_frac: usize, // BOTTOM
    pub sequence: Vec<Track>, // TRACKS
    pub assets: Vec<Asset> // FILES
}

pub struct Asset {
    pub id: u32,
    pub name: String,
    pub src: File,
    pub sample_rate: u32,
    pub duration: u32,
    pub channels: usize
}

pub struct Region {
    pub id: u32,
    pub asset_id: u32,
    pub asset_in: i32,
    pub asset_out: i32,
    pub offset: i32,
}

pub struct Track {
    pub id: u32,
    pub color: Color,
    pub regions: Vec<Region>
}

/*
struct Device {
    inputs: File,
    output: File,
}

struct Route {

}
*/

pub fn write_document(out_file: File, state: TimelineState) {
    println!("WRITING");
}

pub fn read_document(in_file: File) -> TimelineState {

    let asset_file: File = match File::open("examples/test.wav") {
        Ok(f)  => f,
        Err(e) => panic!("{}",  e)
    };

    TimelineState {
        name: "Wowee".to_string(),
        tempo: 127,
        time_beat: 4, // TOP 
        time_frac: 4, // BOTTOM
        sequence: vec![
            Track {
                id: 0,
                color: Color::Light(BaseColor::Yellow),
                regions: vec![
                    Region {
                        id: 0,
                        asset_id: 0,
                        asset_in: 0,
                        asset_out: 448000,
                        offset: 0,

                    }
                ]
            }
        ], // TRACKS
        assets: vec![
            Asset {
                id: 0,
                name: "Audio-1.wav".to_string(),
                src: asset_file,
                sample_rate: 44800,
                duration: 448000,
                channels: 2
            }

        ] // FILES
    }
}