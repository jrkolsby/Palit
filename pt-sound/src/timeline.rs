use sample::{signal, Signal, Sample};
use std::borrow::Borrow;

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen};

pub struct Region<'a> {
    pub wave: WaveFileIterator<'a>,
    pub offset: i32,
    pub duration: i32,
    pub gain: f64,
    pub active: bool,
}

pub struct Timeline<'a> {
    pub wave_file: WaveFileIterator<'a>,
    pub playhead: i32, 
    pub regions: Vec<Region<'a>>,
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

