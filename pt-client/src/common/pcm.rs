extern crate wavefile;
extern crate xmltree;

use std::fs::{self, OpenOptions};
use std::collections::HashMap;

use wavefile::WaveFile;
use itertools::Itertools;
use xmltree::Element;

use crate::common::{Color, Rate};

#[derive(Debug, Clone)]
pub struct Asset {
    pub src: String,
    pub duration: u32,
    pub channels: usize,
    pub waveform: Vec<(u8, u8)>,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub asset_id: u16,
    pub asset_in: u32,
    pub asset_out: u32,
    pub offset: u32,
    pub track: u16,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub mute: bool,
    pub solo: bool,
    pub record: bool,
    pub index: u16,
    pub id: u16,
}

pub fn beat_offset(sample_offset: u32, rate: u32, bpm: u16, zoom: usize) -> u16 {
    // return how many beats passed based on a given sample rate
    let samples_per_beat = (60 * rate) / (bpm as u32);
    (zoom as u32 * (sample_offset / samples_per_beat)) as u16
}

pub fn offset_beat(beats: u16, rate: u32, bpm: u16, zoom: usize) -> u32 {
    let samples_per_beat = (60 * rate) / (bpm as u32);
    beats as u32 * samples_per_beat
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

pub fn generate_waveforms(assets: &mut HashMap<u16, Asset>, 
        rate: u32, tempo: u16, zoom: usize) {
    for (_, asset) in assets.iter_mut() {
        let asset_file = WaveFile::open(asset.src.clone()).unwrap();

        let num_pairs = beat_offset(
            asset.duration, rate, tempo, zoom) as usize;

        let pairs: Vec<(u8, u8)> = file_to_pairs(asset_file, num_pairs, 4);

        asset.waveform = pairs;
    }
}