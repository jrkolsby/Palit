extern crate dsp;
extern crate libc;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;
use std::collections::HashMap;

use sample::signal;

use wavefile::{WaveFile, WaveFileIterator};

use dsp::{NodeIndex, Frame, FromSample, Graph, Node, Sample, Walker};

mod core;
mod midi;
mod synth;
mod tape;
mod action;
mod chord;
mod arpeggio;
mod document;

use crate::core::{event_loop, Module, Frequency};
use crate::document::{Document, read_document};
use crate::action::Action;

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
    //let keys = graph.add_node(Module::DebugKeys(vec![], vec![], 0));
    let keys = graph.add_node(Module::Passthru(vec![]));
    let midi_keys = graph.add_node(Module::Passthru(vec![]));
    let operator = graph.add_node(Module::Passthru(vec![]));
    let octave = graph.add_node(Module::Octave(vec![], 4));

    // Pasting some useful stuff here

    /*
    let synth = graph.add_node(Module::Synth(synth::init()));
    let chord_gen = graph.add_node(Module::Chord(chord::init()));
    let arpeggio = graph.add_node(Module::Arpeggio(arpeggio::init()));
    */

    // Connect keys -> octave -> chord_gen -> synth -> master
    /*
    graph.add_connection(keys, octave);
    graph.add_connection(octave, chord_gen);
    graph.add_connection(chord_gen, synth);
    graph.add_connection(synth, master);
    graph.add_connection(tape, master);
    */

    /*
    graph.add_connection(keys, octave);
    graph.add_connection(octave, arpeggio);

    // Connect arpeggio -> keys -> master
    graph.add_connection(arpeggio, synth);
    graph.add_connection(synth, master);

    // Connect operator to nodes which it controls
    graph.add_connection(operator, tape);
    graph.add_connection(operator, octave);
    graph.add_connection(operator, arpeggio);

    // Set the master node for the graph.
    graph.set_master(Some(master));
    */

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

    let mut operators: HashMap<u16, NodeIndex> = HashMap::new();
    let mut routes: HashMap<u16, NodeIndex> = HashMap::new();

    routes.insert(0, master);

    event_loop(ipc_in, ipc_client, graph, move |mut patch, a| { 
        // ROOT DISPATCH
        match a {
            Action::SetParam(n_id, _, _) => {
                let id = operators.get(&n_id).unwrap();
                patch[*id].dispatch(a)
            },
            Action::NoteOnAt(n_id, _, _) | Action::NoteOnAt(n_id, _, _) => {
                let id = operators.get(&n_id).unwrap();
                patch[*id].dispatch(a);
            },
            Action::NoteOn(_,_) | Action::NoteOff(_) | Action::Octave(_) => {
                let id = operators.get(&104).unwrap();
                patch[*id].dispatch(a)
            },
            Action::Play(n_id) | Action::Stop(n_id) => {
                let id = operators.get(&n_id).unwrap();
                patch[*id].dispatch(a)
            },
            Action::AddRoute(r_id) => {
                let route = patch.add_node(Module::Passthru(vec![]));
                routes.insert(r_id, route);
            },
            Action::OpenProject(name) => {
                *patch = Graph::new();
                let doc = read_document(name);
                for (id, el) in doc.modules.iter() {
                    eprintln!("opened {} with id {:?}", &el.name, id);
                    match &el.name[..] {
                        "timeline" => {
                            let operator = patch.add_node(Module::Passthru(vec![]));
                            while let Some(store) = tape::read(el.to_owned()) {
                                let tape = patch.add_node(Module::Tape(store));
                                patch.add_connection(operator, tape);
                            }
                            operators.insert(*id, operator);
                        },
                        "hammond" => {
                            let operator = patch.add_node(Module::Passthru(vec![]));
                            let store = synth::read(el.to_owned()).unwrap();
                            let instrument = patch.add_node(Module::Synth(store));
                            patch.add_connection(operator, instrument);
                            operators.insert(*id, operator);
                        },
                        "arpeggio" => {
                            let operator = patch.add_node(Module::Passthru(vec![]));
                            let store = arpeggio::read(el.to_owned()).unwrap();
                            let midi_node = patch.add_node(Module::Arpeggio(store));
                            patch.add_connection(operator, midi_node);
                            operators.insert(*id, operator);
                        },
                        "chord" => {
                            let operator = patch.add_node(Module::Passthru(vec![]));
                            let store = chord::read(el.to_owned()).unwrap();
                        },
                        "keyboard" => {
                            let operator = patch.add_node(Module::Passthru(vec![]));
                            let octave = patch.add_node(Module::Octave(vec![], 4));
                            patch.add_connection(operator, octave);
                            operators.insert(*id, operator);
                        },
                        _ => {}
                    }
                }
            },
            Action::MoveRegion(m_id, r_id, track, offset) => {
                if let Some(n_id) = operators.get(&m_id) {
                    if let Some(node) = patch.node_mut(*n_id) {
                        node.dispatch(a)
                    }
                }
            },
            _ => { eprintln!("unimplemented: {:?}", a); }
        }
    })
}
