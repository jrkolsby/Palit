extern crate wavefile;

use wavefile::WaveFile;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: u32,
    pub src: String,
    pub sample_rate: u32,
    pub duration: u32,
    pub channels: usize
}

#[derive(Clone, Debug)]
pub struct Region {
    pub id: u32,
    pub asset_id: u32,
    pub asset_in: i32,
    pub asset_out: i32,
    pub offset: i32,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub id: u32,
    pub regions: Vec<Region>,
}

pub fn file_to_pairs(file: WaveFile, width: usize, height: usize) -> Vec<(i32, i32)> {

    let chunk_size = (file.len()) / (width*2);
    let chunks = &file.iter().chunks(chunk_size);

    let values = chunks.into_iter().map( |chunk| {
        let max = chunk.into_iter().map( |frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(width*2).collect::<Vec<i32>>();

    let global_max = *values.iter().max().unwrap();
    let scale: f64 = height as f64 / global_max as f64;

    let mut pairs = vec![];
    for (i, value) in values.iter().enumerate() {
        if i % 2 > 0 {
            continue;
        }

        let tick: (i32, i32) = (((*value as f64) * scale).round() as i32, 
                                ((values[i+1] as f64) * scale).round() as i32);

        pairs.push(tick);
    }

    pairs
}

/*
struct Device {
    inputs: File,
    output: File,
}

struct Route {

}
*/

/*
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
*/