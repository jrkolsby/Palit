// A quickly made Hammond organ.

extern crate libc;
extern crate alsa;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;

use sample::signal;

use alsa::PollDescriptors;

use wavefile::{WaveFile, WaveFileIterator};

mod core;
mod midi;
mod synth;
mod timeline;

use crate::core::{SF, SigGen, write_samples_io, write_samples_direct, open_audio_dev};
use crate::synth::{Synth};
use crate::timeline::{Region, Timeline};
use crate::midi::{open_midi_dev, read_midi_event, connect_midi_source_ports};

fn arm<'a>(wav: &'a WaveFile, timeline: &'a mut Timeline<'a>) {
    let wav1: WaveFile = WaveFile::open("Who.wav").unwrap();
    for mut region in timeline.regions.iter_mut() {
	region.wave = wav.iter();
    }
}

fn main() -> Result<(), Box<error::Error>> {

    // Blocked by pt-client reader
    println!("Waiting for pt-client...");

    // Configure pt-client IPC
    let mut ipc_client = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-client").unwrap();

    let mut ipc_in = OpenOptions::new()
	.custom_flags(libc::O_NONBLOCK)
	.read(true)
	.open("/tmp/pt-sound").unwrap();

    let mut buf = String::new();

    let (audio_dev, rate) = open_audio_dev()?;

    let midi_dev = open_midi_dev()?;
    let mut midi_input = midi_dev.input();

    let wav1: WaveFile = WaveFile::open("Who.wav").unwrap();
    //let wav2: WaveFile = WaveFile::open("When.wav").unwrap();

    // 256 Voices synth
    let mut synth = Synth {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(rate)),
        stored_sample: None,
        bar_values: [1., 1., 1., 0.75, 0.5, 0., 0., 0., 0.],
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
		offset: 100,
		gain: 1.0,
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
        out: ipc_client,
    };

    // Create an array of file descriptors to poll
    let mut fds = audio_dev.get()?;
    fds.append(&mut (&midi_dev, Some(alsa::Direction::Capture)).get()?); 
    
    // Use direct-mode memory mapping for minimum overhead
    let mut mmap = audio_dev.direct_mmap_playback::<SF>();
    
    // if direct-mode unavailable, use mmap emulation instead
    let mut io = if mmap.is_err() {
        Some(audio_dev.io_i16()?)
    } else { None };

    let mut playing: bool = true;

    loop {
	if playing {
	    if let Ok(ref mut mmap) = mmap {
		if write_samples_direct(&audio_dev, mmap, &mut synth)? { continue; }
	    } else if let Some(ref mut io) = io {
		if write_samples_io(&audio_dev, io, &mut synth)? { continue; }
	    }
	}

	if read_midi_event(&mut midi_input, &mut synth)? { continue; }

	buf = String::new();
	ipc_in.read_to_string(&mut buf);
	match &buf[..] {
	    "OPEN_PROJECT" => { println!("OPEN"); },
	    "PLAY" => { playing = true; },
	    "STOP" => { playing = false; }
            "C1_ON" =>  { synth.add_note(86, 0.5); }
            "D1_ON" =>  { synth.add_note(89, 0.5); }
            "E1_ON" =>  { synth.add_note(92, 0.5); }
            "C1_OFF" => { synth.remove_note(86); }
            "D1_OFF" => { synth.remove_note(89); }
            "E1_OFF" => { synth.remove_note(92); }
	    _ => {}
	}

        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}
