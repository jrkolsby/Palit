extern crate dsp;
extern crate libc;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;

use sample::signal;

use wavefile::{WaveFile, WaveFileIterator};

use dsp::{sample::ToFrameSliceMut, Frame, FromSample, Graph, Node, Sample, Walker};

mod core;
mod midi;
mod synth;
mod timeline;
mod mixer;

use crate::core::{event_loop, DspNode, Frequency};
use crate::synth::{Synth};
use crate::timeline::{Region, Timeline};
use crate::mixer::{Mixer};

fn arm<'a>(wav: &'a WaveFile, timeline: &'a mut Timeline<'a>) {
    let wav1: WaveFile = WaveFile::open("Who.wav").unwrap();
    for mut region in timeline.regions.iter_mut() {
	region.wave = wav.iter();
    }
}

const A5_HZ: Frequency = 440.0;
const D5_HZ: Frequency = 587.33;
const F5_HZ: Frequency = 698.46;

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

    let wav1: WaveFile = WaveFile::open("Who.wav").unwrap();
    //let wav2: WaveFile = WaveFile::open("When.wav").unwrap();

    // Hammond synth
    let mut synth = Synth {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(48000)),
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
        playing: false,
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
        //out: ipc_client,
    };

    let mut root = Mixer {
        timeline: tl,
        synths: vec![synth],
    };
    
    // Construct our dsp graph.
    let mut graph = Graph::new();

    // Construct our fancy Synth and add it to the graph!
    let synth = graph.add_node(DspNode::Synth);

    // Connect a few oscillators to the synth.
    let (_, oscillator_a) = graph.add_input(DspNode::Oscillator(0.0, A5_HZ, 0.2), synth);
    graph.add_input(DspNode::Oscillator(0.0, D5_HZ, 0.1), synth);
    graph.add_input(DspNode::Oscillator(0.0, F5_HZ, 0.15), synth);

    // If adding a connection between two nodes would create a cycle, Graph will return an Err.
    if let Err(err) = graph.add_connection(synth, oscillator_a) {
        println!(
            "Testing for cycle error: {:?}",
            std::error::Error::description(&err)
        );
    }

    event_loop(ipc_in, ipc_client, graph, synth)
}
