use sample::{signal, Signal, Sample};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;

use dsp::{Node, FromSample, Frame};

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen, CHANNELS, Output, Module};
use crate::action::Action;

pub struct Region<'a> {
    pub wave: WaveFileIterator<'a>,
    pub offset: u32,
    pub duration: u32,
    pub gain: f32,
    pub active: bool,
}

pub struct Timeline<'a> {
    pub bpm: u16,
    pub time_beat: usize,
    pub time_note: usize,
    pub loop_on: bool,
    pub loop_in: u32,
    pub loop_out: u32,
    pub duration: u32,
    pub playhead: u32, 
    pub regions: Vec<Region<'a>>,
    //pub out: File,
    pub playing: bool,
}

impl<'a> Timeline<'a> {
    //fn new() -> Self {}
    fn arm(&mut self, t: u32) {
        self.playhead == t;
        for region in self.regions.iter_mut() {
            //if region.offset < t and 
        }
    }
}

impl Module for Timeline<'_> {
    fn dispatch(&mut self, a: Action, outputs: Vec<Box<Module>>) -> Action {
        match a {
            _ => Action::Noop
        }
    }
}

impl Node<[Output; CHANNELS]> for Timeline<'_> {
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        dsp::slice::map_in_place(buffer, |_| {
            self.playhead += 1;
            if self.playhead % 65536 == 0 {
                println!("tick!");
                //self.out.write(b"TICK");
            }
            let mut z: f32 = 0.0;
            if !self.playing {
                return Frame::from_fn(|_| z);
            }
            // see iter() iter_mut() and into_iter()
            for region in self.regions.iter_mut() {
                if self.playhead == region.offset {
                    region.active = true;
                }
                if region.active {
                    if self.playhead >= region.offset + region.duration {
                        println!("stop");
                        region.active = false;
                    } else {
                        let x: f32 = region.wave.next().unwrap()[0] as f32 * 0.000001;
                        z += x * region.gain;
                    }
                }
            }
            let z = z.min(0.999).max(-0.999);
            Frame::from_fn(|_| z)
        })
    }
}

