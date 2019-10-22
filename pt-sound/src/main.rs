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
mod action;

use crate::core::{event_loop, Module, Frequency};

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

    // Construct our dsp graph.
    let mut graph = Graph::new();

    // Construct Master
    let master = graph.add_node(Module::Master);

    // Construct special event nodes
    let keys = graph.add_node(Module::DebugKeys(vec![], vec![], 48000));
    let midi_keys = graph.add_node(Module::Passthru(vec![]));

    let timeline = graph.add_node(Module::Timeline(timeline::init()));

    let synth = graph.add_node(Module::Synth(synth::Store {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(48000)),
        stored_sample: None,
        bar_values: [1., 1., 1., 0.75, 0.5, 0., 0., 0., 0.],
    }));

    // Connect keys -> synth -> master
    graph.add_connection(keys, synth);
    graph.add_connection(synth, master);
    graph.add_connection(timeline, master);

    // Set the master node for the graph.
    graph.set_master(Some(master));

    /*
    // Pasting some useful stuff here

    // Connect a few oscillators to the synth.
    graph.add_input(Module::Oscillator(0.0, A5_HZ, 0.2), master);
    graph.add_input(Module::Oscillator(0.0, D5_HZ, 0.1), master);
    graph.add_input(Module::Oscillator(0.0, F5_HZ, 0.15), master);

    if let Err(err) = graph.add_connection(master, oscillator_a) {
        println!(
            "Testing for cycle error: {:?}",
            std::error::Error::description(&err)
        );
    }

    let mut inputs = patch.inputs(master);
    while let Some(input_idx) = inputs.next_node(&patch) {
        if let Module::Oscillator(_, ref mut pitch, _) = patch[input_idx] {
            // Pitch down our oscillators for fun.
            *pitch -= 0.1;
        }
    }
    */

    event_loop(ipc_in, ipc_client, graph, midi_keys, keys, |a| { a })
}
