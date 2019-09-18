extern crate wavefile;
extern crate xmltree;

use std::fs;

use wavefile::WaveFile;
use itertools::Itertools;
use xmltree::Element;

use crate::common::{Color, Rate};

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: u32,
    pub src: String,
    pub duration: u32,
    pub channels: usize
}

#[derive(Clone, Debug)]
pub struct Region {
    pub id: u32,
    pub asset_id: u32,
    pub asset_in: u32,
    pub asset_out: u32,
    pub offset: u32,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub id: u32,
    pub color: Color,
    pub regions: Vec<Region>,
}

#[derive(Clone, Debug)]
pub struct TimelineState {
    pub name: String,
    pub tempo: u16,             // TEMPO
    pub time_beat: usize,       // TOP 
    pub time_note: usize,       // BOTTOM
    pub duration_measure: usize,
    pub duration_beat: usize,
    pub zoom: usize,              // BEATS per tick
    pub loop_mode: bool,        // TRUE FOR LOOP
    pub sequence: Vec<Track>,   // TRACKS
    pub assets: Vec<Asset>,      // FILES
    pub sample_rate: Rate,

    pub tick: bool,

    pub scroll_x: u16,
    pub scroll_y: u16,
    pub focus: usize, 

    pub playhead: u16,
}

fn sample_rate(rate: Rate) -> u32 {
    match rate {
        Rate::Fast => 64000,
        Rate::Med => 48000,
        Rate::Slow => 32000,
    }
}

pub fn beat_offset(sample_offset: u32, srate: Rate, bpm: u16, zoom: usize) -> u32 {
    // return how many beats passed based on a given sample rate
    let samples_per_beat = (60 * sample_rate(srate)) / (bpm as u32);
    sample_offset / (samples_per_beat * zoom as u32)
}

pub fn file_to_pairs(file: WaveFile, width: usize, samples_per_tick: u16) -> Vec<(i32, i32)> {

    let chunk_size = (file.len()) / (width*2);
    let chunks = &file.iter().chunks(chunk_size);

    let values = chunks.into_iter().map( |chunk| {
        let max = chunk.into_iter().map( |frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(width*2).collect::<Vec<i32>>();

    let global_max = *values.iter().max().unwrap();
    let scale: f64 = 4.0 / global_max as f64;

    let mut pairs = vec![];
    for (i, value) in values.iter().enumerate() {
        if i % 2 > 0 { continue; }

        let tick: (i32, i32) = (((*value as f64) * scale).round() as i32, 
                                ((values[i+1] as f64) * scale).round() as i32);

        pairs.push(tick);
    }

    pairs
}

/*
pub fn write_document(out_file: File, state: TimelineState) {
    println!("WRITING");
}
*/

pub fn read_document(in_file: String) -> TimelineState {

    /*
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.xml").unwrap();
        */

    let doc_str: String = fs::read_to_string(in_file).unwrap();
    let doc: Element = Element::parse(doc_str.as_bytes()).unwrap();
    
    // SECTIONS
    let format: &Element = doc.get_child("format").unwrap();
    let tempo: &Element = doc.get_child("tempo").unwrap();
    let assets: &Element = doc.get_child("assets").unwrap();
    let tracks: &Element = doc.get_child("tracks").unwrap();

    let mut state = TimelineState {
	name: "Wowee".to_string(),
	tempo: 127,
	time_beat: 4, // TOP 
	time_note: 4, // BOTTOM
	duration_beat: 0,
	duration_measure: 15,
	zoom: 1,
	loop_mode: false,
	focus: 0,
	scroll_x: 0,
	scroll_y: 0,
	tick: true,
	playhead: 0,
	sample_rate: Rate::Fast,
        sequence: vec![], // TRACKS
        assets: vec![] // FILES
    };

    // GET FORMAT
    let bitrate = format.attributes.get("bitrate").unwrap();
    let samplerate = format.attributes.get("samplerate").unwrap();

    // GET TEMPO
    let bpm = tempo.attributes.get("bpm").unwrap();
    let note = tempo.attributes.get("note").unwrap();
    let beat = tempo.attributes.get("beat").unwrap();

    // GET ASSETS
    for (i, asset) in assets.children.iter().enumerate() {
	let mut id: &str = asset.attributes.get("id").unwrap();
	state.assets.push(Asset {
	    id: id[1..].parse().unwrap(),
	    src: asset.attributes.get("src").unwrap().parse().unwrap(),
	    duration: 48000, 	// TODO
	    channels: 2,	// TODO
	})
    }

    // GET TRACKS
    for track in tracks.children.iter() {
        eprintln!("color {:}", track.attributes.get("color").unwrap());
        for region in track.children.iter() {
            eprintln!("asset {:}", region.attributes.get("asset").unwrap());
            eprintln!("offset {:}", region.attributes.get("offset").unwrap());
        }
    }

    state
}
