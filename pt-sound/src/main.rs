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
	    "STOP" => { playing = false; },

            "C1_ON" =>  { synth.add_note(69, 0.5); },
            "C1#_ON" => { synth.add_note(70, 0.5); },
            "D1_ON" => { synth.add_note(71, 0.5); },
            "D1#_ON" => { synth.add_note(72, 0.5); },
            "E1_ON" => { synth.add_note(73, 0.5); },
            "F1_ON" => { synth.add_note(74, 0.5); },
            "F1#_ON" => { synth.add_note(75, 0.5); },
            "G1_ON" => { synth.add_note(76, 0.5); },
            "G1#_ON" => { synth.add_note(77, 0.5); },
            "A1_ON" => { synth.add_note(78, 0.5); },
            "A1#_ON" => { synth.add_note(79, 0.5); },
            "B1_ON" => { synth.add_note(80, 0.5); },
            "C2_ON" =>  { synth.add_note(81, 0.5); },
            "C2#_ON" => { synth.add_note(82, 0.5); },
            "D2_ON" => { synth.add_note(83, 0.5); },
            "D2#_ON" => { synth.add_note(84, 0.5); },
            "E2_ON" => { synth.add_note(85, 0.5); },
            "F2_ON" => { synth.add_note(86, 0.5); },
            "F2#_ON" => { synth.add_note(87, 0.5); },
            "G2_ON" => { synth.add_note(88, 0.5); },
            "G2#_ON" => { synth.add_note(89, 0.5); },
            "A2_ON" => { synth.add_note(90, 0.5); },
            "A2#_ON" => { synth.add_note(91, 0.5); },
            "B2_ON" => { synth.add_note(92, 0.5); },
            "C3_ON" =>  { synth.add_note(93, 0.5); },
            "C3#_ON" => { synth.add_note(94, 0.5); },
            "D3_ON" => { synth.add_note(95, 0.5); },
            "D3#_ON" => { synth.add_note(96, 0.5); },
            "E3_ON" => { synth.add_note(97, 0.5); },
            "F3_ON" => { synth.add_note(98, 0.5); },
            "F3#_ON" => { synth.add_note(99, 0.5); },
            "G3_ON" => { synth.add_note(100, 0.5); },
            "G3#_ON" => { synth.add_note(101, 0.5); },
            "A3_ON" => { synth.add_note(102, 0.5); },
            "A3#_ON" => { synth.add_note(103, 0.5); },
            "B3_ON" => { synth.add_note(104, 0.5); },
            "C1_OFF" =>  { synth.remove_note(69); },
            "C1#_OFF" => { synth.remove_note(70); },
            "D1_OFF" => { synth.remove_note(71); },
            "D1#_OFF" => { synth.remove_note(72); },
            "E1_OFF" => { synth.remove_note(73); },
            "F1_OFF" => { synth.remove_note(74); },
            "F1#_OFF" => { synth.remove_note(75); },
            "G1_OFF" => { synth.remove_note(76); },
            "G1#_OFF" => { synth.remove_note(77); },
            "A1_OFF" => { synth.remove_note(78); },
            "A1#_OFF" => { synth.remove_note(79); },
            "B1_OFF" => { synth.remove_note(80); },
            "C2_OFF" =>  { synth.remove_note(81); },
            "C2#_OFF" => { synth.remove_note(82); },
            "D2_OFF" => { synth.remove_note(83); },
            "D2#_OFF" => { synth.remove_note(84); },
            "E2_OFF" => { synth.remove_note(85); },
            "F2_OFF" => { synth.remove_note(86); },
            "F2#_OFF" => { synth.remove_note(87); },
            "G2_OFF" => { synth.remove_note(88); },
            "G2#_OFF" => { synth.remove_note(89); },
            "A2_OFF" => { synth.remove_note(90); },
            "A2#_OFF" => { synth.remove_note(91); },
            "B2_OFF" => { synth.remove_note(92); },
            "C3_OFF" =>  { synth.remove_note(93); },
            "C3#_OFF" => { synth.remove_note(94); },
            "D3_OFF" => { synth.remove_note(95); },
            "D3#_OFF" => { synth.remove_note(96); },
            "E3_OFF" => { synth.remove_note(97); },
            "F3_OFF" => { synth.remove_note(98); },
            "F3#_OFF" => { synth.remove_note(99); },
            "G3_OFF" => { synth.remove_note(100); },
            "G3#_OFF" => { synth.remove_note(101); },
            "A3_OFF" => { synth.remove_note(102); },
            "A3#_OFF" => { synth.remove_note(103); },
            "B3_OFF" => { synth.remove_note(104); },
	    _ => {}
	}

        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}
