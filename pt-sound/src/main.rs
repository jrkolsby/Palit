// A quickly made Hammond organ.

extern crate alsa;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use sample::signal;

use alsa::PollDescriptors;

use wavefile::{WaveFile, WaveFileIterator};

mod core;
mod synth;
mod timeline;

use crate::core::{SF, SigGen, write_samples_io, write_samples_direct, open_audio_dev};
use crate::synth::{Synth};
use crate::timeline::{Region, Timeline};

fn main() -> Result<(), Box<error::Error>> {
    let (audio_dev, rate) = open_audio_dev()?;

    let wav1: WaveFile = WaveFile::open("Who.wav").unwrap();
    //let wav2: WaveFile = WaveFile::open("When.wav").unwrap();

    // 256 Voices synth
    let mut synth = Synth {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(rate)),
        stored_sample: None,
        bar_values: [1., 0.75, 1., 0.75, 0., 0., 0., 0., 0.75],
    };

    let mut tl = Timeline {
	bpm: 127,
	duration: 960000,
	time_beat: 4,
	time_note: 4,
	loop_on: false,
	loop_in: 0,
	loop_out: 0,
	playhead: 0,
	regions: vec![
	    Region {
		active: false,
		offset: 0,
		gain: 0.1,
		duration: 480000,
		wave: wav1.iter(),
	    },
	    Region {
		active: false,
		gain: 1.0,
		offset: 1320000,
		duration: 480000,
		wave: wav1.iter(),
	    }
	],
    };

    // Create an array of file descriptors to poll
    let mut fds = audio_dev.get()?;
    
    // Use direct-mode memory mapping for minimum overhead
    let mut mmap = audio_dev.direct_mmap_playback::<SF>();
    
    // if direct-mode unavailable, use mmap emulation instead
    let mut io = if mmap.is_err() {
        Some(audio_dev.io_i16()?)
    } else { None };

    // Play minor 7
    synth.add_note(86, 0.5);
    synth.add_note(89, 0.5);
    synth.add_note(92, 0.5);

    loop {
        if let Ok(ref mut mmap) = mmap {
            if write_samples_direct(&audio_dev, mmap, &mut synth)? { continue; }
        } else if let Some(ref mut io) = io {
            if write_samples_io(&audio_dev, io, &mut synth)? { continue; }
        }
        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}
