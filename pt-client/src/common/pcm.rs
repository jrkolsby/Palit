extern crate xmltree;

use std::fs::{self, OpenOptions};
use std::collections::HashMap;
use std::io::BufReader;

use hound;
use itertools::{self, Itertools};
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
    pub record: u8,
    pub monitor: bool,
    pub index: u16,
    pub id: u16,
}

pub fn char_offset(sample_offset: u32, rate: u32, bpm: u16, zoom: usize) -> u16 {
    // return how many beats passed based on a given sample rate
    let samples_per_beat = (60 * rate) / (bpm as u32);
    (zoom as u32 * (sample_offset / samples_per_beat)) as u16
}

pub fn offset_char(beats: u16, rate: u32, bpm: u16, zoom: usize) -> u32 {
    let samples_per_beat = (60 * rate) / (bpm as u32);
    beats as u32 * samples_per_beat
}

pub fn generate_partial_waveform(mut file: String, tail_len: u32, rate: u32, tempo: u16, zoom: usize) -> Vec<(u8, u8)> {
    let asset_file = match hound::WavReader::open(file) {
        Ok(a) => a,
        Err(_) => return vec![]
    };
    let num_pairs = char_offset(tail_len, rate, tempo, zoom) as usize;
    let pairs = generate_waveform(asset_file, num_pairs);
    return pairs;
}

pub fn generate_waveform(mut file: hound::WavReader<BufReader<std::fs::File>>, width: usize) -> Vec<(u8, u8)> {
    if width == 0 { return vec![]; }

    let channels = file.spec().channels as usize;
    let chunk_size = match file.duration() as usize / (width * 2) {
        0 => return vec![],
        n => n * channels
    };

    let chunks: &itertools::IntoChunks<hound::WavSamples<'_, std::io::BufReader<std::fs::File>, i32>> = &file.samples().chunks(chunk_size);

    let values = chunks.into_iter().map(|chunk| {
        let max = chunk.into_iter().map(|frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(width * 2).collect::<Vec<i32>>();

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
        let asset_file = hound::WavReader::open(asset.src.clone()).unwrap();

        let num_pairs = char_offset(
            asset.duration, rate, tempo, zoom) as usize;

        let pairs: Vec<(u8, u8)> = generate_waveform(asset_file, num_pairs);

        asset.waveform = pairs;
    }
}