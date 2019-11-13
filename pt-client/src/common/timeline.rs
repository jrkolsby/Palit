extern crate wavefile;
extern crate xmltree;

use std::fs::{self, OpenOptions};
use std::collections::HashMap;

use wavefile::WaveFile;
use itertools::Itertools;
use xmltree::Element;

use crate::common::{Color, Rate};

const PALIT_ROOT: &str = "/usr/local/palit/";

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: u16,
    pub src: String,
    pub duration: u32,
    pub channels: usize,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub id: u16,
    pub asset_id: u16,
    pub asset_in: u32,
    pub asset_out: u32,
    pub offset: u32,
    pub track: u16,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub id: u16,
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

    // This isn't ideal, especially if we have a lot of waveforms,
    // But the nature of our 'document object' (multifocus) only
    // allows us to render focii as a function of state. We'll see
    // if this is a horrible bottleneck...
    pub waveforms: HashMap<u16, Vec<(u8, u8)>>,
    pub regions: HashMap<u16, Region>,

    pub tick: bool,

    pub scroll_x: u16,
    pub scroll_y: u16,

    pub playhead: u16,

    pub focus: (usize, usize),
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

pub fn file_to_pairs(file: WaveFile, width: usize, samples_per_tick: u16) -> Vec<(u8, u8)> {

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

        let tick: (u8, u8) = (((*value as f64) * scale).round() as u8, 
                                ((values[i+1] as f64) * scale).round() as u8);

        pairs.push(tick);
    }

    pairs
}

pub fn write_document(out_file: String, state: TimelineState) {
    let fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(out_file).unwrap();
}

pub fn read_document(in_file: String) -> TimelineState {

    /*
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.xml").unwrap();
        */

    eprintln!("read_document: {}", in_file);

    let doc_str: String = fs::read_to_string(format!("{}{}", PALIT_ROOT, in_file)).unwrap();
    let doc: Element = Element::parse(doc_str.as_bytes()).unwrap();
    
    // sections
    let format: &Element = doc.get_child("format").unwrap();
    let tempo: &Element = doc.get_child("tempo").unwrap();
    let meta: &Element = doc.get_child("meta").unwrap();
    let duration: &Element = doc.get_child("duration").unwrap();
    let assets: &Element = doc.get_child("assets").unwrap();
    let tracks: &Element = doc.get_child("tracks").unwrap();

    // get format
    let bitrate = format.attributes.get("bitrate").unwrap();
    let samplerate = format.attributes.get("samplerate").unwrap();

    // get tempo
    let bpm = tempo.attributes.get("bpm").unwrap();
    let note = tempo.attributes.get("note").unwrap();
    let beat = tempo.attributes.get("beat").unwrap();

    // get metadata 
    let title = meta.attributes.get("title").unwrap();

    // get duration
    let duration_measure = duration.attributes.get("measure").unwrap();
    let duration_beat = duration.attributes.get("beat").unwrap();

    let mut state = TimelineState {
        name: title.to_string(),
        tempo: bpm.parse().unwrap(),
        time_beat: beat.parse().unwrap(), // TOP 
        time_note: note.parse().unwrap(), // BOTTOM
        duration_measure: duration_measure.parse().unwrap(),
        duration_beat: duration_beat.parse().unwrap(),
        zoom: 1,
        loop_mode: false,
        scroll_x: 0,
        scroll_y: 0,
        tick: true,
        playhead: 0,
        sample_rate: Rate::Fast,
        sequence: vec![], // TRACKS
        assets: vec![], // FILES
        regions: HashMap::new(), 
        waveforms: HashMap::new(),
        focus: (0,0),
    };

    // GET ASSETS
    for asset in assets.children.iter() {
        let id: &str = asset.attributes.get("id").unwrap();
        let duration: &str = asset.attributes.get("size").unwrap();
        state.assets.push(Asset {
            id: id[1..].parse().unwrap(),
            src: asset.attributes.get("src").unwrap().parse().unwrap(),
            duration: duration.parse().unwrap(),
            channels: 2,	// TODO
        })
    }

    // GET TRACKS
    for (i, track) in tracks.children.iter().enumerate() {
        let t_id: &str = track.attributes.get("id").unwrap();
        let col: &str = track.attributes.get("color").unwrap();
        // Match color type
        let color: Color = match col {
            "yellow" => Color::Yellow,
            "pink" => Color::Pink,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => Color::Red,
        };
        // Create new region array and populate
        let mut regions: Vec<Region> = vec![];
        for region in track.children.iter() {

            // Get region info
            let r_id: &str = region.attributes.get("id").unwrap();
            let a_id: &str = region.attributes.get("asset").unwrap();
            let offset: &str = region.attributes.get("offset").unwrap();
            let a_in: &str = region.attributes.get("in").unwrap();
            let a_out: &str = region.attributes.get("out").unwrap();

            let new_region = Region {
                id: r_id[1..].parse().unwrap(),
                asset_id: a_id[1..].parse().unwrap(),
                asset_in: a_in.parse().unwrap(),
                asset_out: a_out.parse().unwrap(),
                offset: offset.parse().unwrap(),
                track: i as u16,
            };

            state.regions.insert(new_region.id, new_region.clone());
            regions.push(new_region);
        }

        state.sequence.push(Track {
            id: t_id[1..].parse().unwrap(),
            color,
            regions,
        })
    }

    state
}
