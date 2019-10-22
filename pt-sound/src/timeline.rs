use sample::{signal, Signal, Sample};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen};
use crate::action::Action;
use crate::core::{Output};

pub struct Region {
    pub buffer: Vec<Output>,
    pub offset: u32,
    pub duration: u32,
    pub gain: f32,
}

pub struct Store {
    pub bpm: u16,
    pub time_beat: usize,
    pub time_note: usize,
    pub loop_on: bool,
    pub loop_in: u32,
    pub loop_out: u32,
    pub duration: u32,
    pub playhead: u32, 
    pub regions: Vec<Region>,
    pub playing: bool,
}

pub fn init() -> Store {
    let mut wav_f = WaveFile::open("Who.wav").unwrap();
    let mut wav_iter = wav_f.iter();
    let mut buf: Vec<f32> = Vec::new();
    while let Some(s) = wav_iter.next() {
        // s is a i32
        buf.push(s[0] as f32 / 0.000001);
    };
    return Store {
        bpm: 127,
        duration: 960000,
        time_beat: 4,
        time_note: 4,
        loop_on: false,
        loop_in: 0,
        loop_out: 0,
        playhead: 0,
        playing: false,
        regions: vec![
            Region {
                offset: 100,
                gain: 1.0,
                duration: 480000,
                buffer: buf.clone(),
            },
            Region {
                gain: 1.0,
                offset: 1320000,
                duration: 480000,
                buffer: buf.clone(),
            }
        ],
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Actions for outputs
        Option<Vec<Action>>, // Actions for inputs
        Option<Vec<Action>> // Actions for client
    ) {
        if store.playhead % 65536 == 0 {
            (None, None, Some(vec![Action::Tick]))
        } else {
            (None, None, None)
        }
}

pub fn compute(store: &mut Store) -> Output {
    store.playhead += 1;
    if store.playhead % 65536 == 0 {
        println!("tick!");
        //self.out.write(b"TICK");
    }
    let mut z: f32 = 0.0;
    if !store.playing {
        return z
    }
    // see iter() iter_mut() and into_iter()
    for region in store.regions.iter_mut() {
        if store.playhead >= region.offset && store.playhead < region.offset + region.duration {
            let index = (store.playhead-region.offset) as usize;
            let x: f32 = region.buffer[index];
            z += x * region.gain;
        }
    }
    let z = z.min(0.999).max(-0.999);
    z
}