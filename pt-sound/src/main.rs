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
mod chord;
mod arpeggio;

use crate::core::{event_loop, Module, Frequency};

const A5_HZ: Frequency = 440.0;
const D5_HZ: Frequency = 587.33;
const F5_HZ: Frequency = 698.46;

fn main() -> Result<(), Box<error::Error>> {

    // Blocked by pt-client reader
    println!("Waiting for pt-client...");

    // Configure pt-client IPC
    let mut ipc_client = match OpenOptions::new() 
        .write(true)
        .open("/tmp/pt-client") {
            Ok(a) => a,
            Err(_) => panic!("Could not open /tmp/pt-client")
        };

    let mut ipc_in = match OpenOptions::new()
        .custom_flags(libc::O_NONBLOCK)
        .read(true)
        .open("/tmp/pt-sound") {
            Ok(a) => a,
            Err(_) => panic!("Could not open /tmp/pt-sound")
        };

    // Construct our dsp graph.
    let mut graph = Graph::new();

    // Construct Master
    let master = graph.add_node(Module::Master);

    // Construct special event nodes
    let keys = graph.add_node(Module::DebugKeys(vec![], vec![], 10000));
    //let keys = graph.add_node(Module::Passthru(vec![]));
    let midi_keys = graph.add_node(Module::Passthru(vec![]));
    let operator = graph.add_node(Module::Passthru(vec![]));
    let octave = graph.add_node(Module::Octave(vec![], 4));

    let timeline = graph.add_node(Module::Timeline(timeline::init()));
    let synth = graph.add_node(Module::Synth(synth::init()));
    let chord_gen = graph.add_node(Module::Chord(chord::init()));
    let arpeggio = graph.add_node(Module::Arpeggio(arpeggio::init()));

    // Connect keys -> octave -> chord_gen -> synth -> master
    /*
    graph.add_connection(keys, octave);
    graph.add_connection(octave, chord_gen);
    graph.add_connection(chord_gen, synth);
    graph.add_connection(synth, master);
    graph.add_connection(timeline, master);
    */

    // Connect arpeggio -> keys -> master
    graph.add_connection(arpeggio, synth);
    graph.add_connection(synth, master);

    // Connect operator to nodes which it controls
    graph.add_connection(operator, timeline);
    graph.add_connection(operator, octave);
    graph.add_connection(operator, arpeggio);

    // Set the master node for the graph.
    graph.set_master(Some(master));

    // Pasting some useful stuff here

    /*
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

    event_loop(ipc_in, ipc_client, graph, operator, midi_keys, keys, |a| { a })
}
