use sample::{signal, Signal, Sample};
use std::borrow::Borrow;

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen};

pub struct Region<'a> {
    pub wave: WaveFileIterator<'a>,
    pub offset: u32,
    pub duration: u32,
    pub gain: f64,
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
}

impl Timeline<'_> {
    //fn new() -> Self {}
    fn arm(&mut self, t: u32) {
	self.playhead == t;
	for region in self.regions.iter_mut() {
	    //if region.offset < t and 
	}
    }
    fn stop() {}
}

impl Iterator for Timeline<'_> { 
    type Item = SF;
    fn next(&mut self) -> Option<Self::Item> {
	self.playhead += 1;
	let mut z: f64 = 0.0;
	// see iter() iter_mut() and into_iter()
	for region in self.regions.iter_mut() {
	    if self.playhead == region.offset {
		println!("play");
		region.active = true;
	    }
	    if region.active {
		if self.playhead >= region.offset + region.duration {
		    println!("stop");
		    region.active = false;
		} else {
		    let x: f64 = 
			region.wave.next().unwrap()[0] as f64 * 0.000001;
		    z += x * region.gain;
		}
	    }
	}
        let z = z.min(0.999).max(-0.999);
        let z: Option<SF> = Some(SF::from_sample(z));
        z
    }
}

