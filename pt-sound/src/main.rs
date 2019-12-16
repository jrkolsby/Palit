extern crate dsp;
extern crate libc;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::io::prelude::*;
use std::collections::HashMap;
use std::borrow::BorrowMut;

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

use crate::core::{event_loop, Module, Frequency, Key};
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

    // Pasting some useful stuff heref

    /*
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

    event_loop(ipc_in, ipc_client, graph, move |mut patch, a| { 
        // ROOT DISPATCH
        eprintln!("ACTION {:?}", a);
        match a {
            Action::SetParam(n_id, _, _) => {
                if let Some(id) = operators.get(&n_id) {
                    patch[*id].dispatch(a)
                }
            },
            Action::NoteOnAt(n_id, _, _) | Action::NoteOffAt(n_id, _) => {
                if let Some(id) = operators.get(&n_id) {
                    patch[*id].dispatch(a);
                }
            },
            Action::NoteOn(_,_) | Action::NoteOff(_) | Action::Octave(_) => {
                if let Some(id) = operators.get(&104) {
                    patch[*id].dispatch(a)
                }
            },
            Action::Play(n_id) | Action::Stop(n_id) => {
                if let Some(id) = operators.get(&n_id) {
                    patch[*id].dispatch(a)
                }
            },
            Action::AddRoute(r_id) => {
                let route = patch.add_node(Module::Passthru(vec![]));
                routes.insert(r_id, route);
            },
            Action::PatchIn(n_id, in_id, r_id) => {
                if let Some(route) = routes.get(&r_id) {
                    let module = match &patch[*operators.get(&n_id).unwrap()] {
                        Module::Operator(_, inputs, _) => {
                            patch.add_connection(*route, inputs[in_id]);
                        }
                        _ => {}
                    };
                }
            },
            Action::PatchOut(n_id, out_id, r_id) => {
                if let Some(route) = routes.get(&r_id) {
                    let module = match &patch[*operators.get(&n_id).unwrap()] {
                        Module::Operator(_, _, outputs) => {
                            patch.add_connection(outputs[out_id], *route);
                        }
                        _ => {}
                    };
                }
            },
            Action::MoveRegion(m_id, r_id, track, offset) => {
                if let Some(n_id) = operators.get(&m_id) {
                    if let Some(node) = patch.node_mut(*n_id) {
                        node.dispatch(a)
                    }
                }
            },
            Action::OpenProject(name) => {
                *patch = Graph::new();
                operators = HashMap::new();
                routes = HashMap::new();
                let mut doc = read_document(name);
                for (id, el) in doc.modules.iter_mut() {
                    match &el.name[..] {
                        "timeline" => {
                            let mut inputs: Vec<NodeIndex> = vec![];
                            let mut outputs: Vec<NodeIndex> = vec![];
                            while let Some(store) = tape::read(el) {
                                let tape = patch.add_node(Module::Tape(store));
                                inputs.push(tape);
                                outputs.push(tape);
                            }
                            let operator = patch.add_node(Module::Operator(vec![], 
                                inputs.clone(), 
                                outputs.clone())
                            );
                            for input in inputs.iter() {
                                patch.add_connection(operator, *input);
                            }
                            operators.insert(*id, operator);
                        },
                        "hammond" => {
                            let store = match synth::read(el) {
                                Some(a) => a,
                                None => panic!("Invalid module {}", id)
                            };
                            let instrument = patch.add_node(Module::Synth(store));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![instrument], // INS
                                vec![instrument]) // OUTS
                            );
                            patch.add_connection(operator, instrument);
                            operators.insert(*id, operator);
                        },
                        "arpeggio" => {
                            let store = match arpeggio::read(el) {
                                Some(a) => a,
                                None => panic!("Invalid module {}", id)
                            };
                            let inst = patch.add_node(Module::Arpeggio(store));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![inst], 
                                vec![inst])
                            );
                            patch.add_connection(operator, inst);
                            operators.insert(*id, operator);
                        },
                        "chord" => {
                            let store = chord::read(el).unwrap();
                            let inst = patch.add_node(Module::Chord(store));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![inst], 
                                vec![inst])
                            );
                            patch.add_connection(operator, inst);
                            operators.insert(*id, operator);
                        },
                        "keyboard" => {
                            let shift = el.attributes.get("octave").unwrap();
                            let _shift = shift.parse::<Key>().unwrap();
                            let octave = patch.add_node(Module::Octave(vec![], _shift));
                            //let shift = patch.add_node(Module::Octave(vec![], 4));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![octave], 
                                vec![octave])
                            );
                            patch.add_connection(operator, octave);
                            operators.insert(*id, operator);
                        },
                        // This module should always be last in doc.modules
                        "patch" => {
                            while let Some(mut route_el) = el.take_child("route") {
                                let r_id = route_el.attributes.get("id").unwrap();
                                let _r_id = r_id.parse::<u16>().unwrap();
                                let route = patch.add_node(Module::Passthru(vec![]));
                                routes.insert(_r_id, route);
                                while let Some(input) = route_el.take_child("input") {
                                    let m_id = input.attributes.get("module").unwrap();
                                    let _m_id = m_id.parse::<u16>().unwrap();

                                    let io_id = input.attributes.get("channel").unwrap();
                                    let _io_id = io_id.parse::<usize>().unwrap() - 1;
                                    
                                    let op_id = operators.get(&_m_id).unwrap();

                                    let in_id = match &patch[*op_id] {
                                        Module::Operator(_, ins, _) => ins[_io_id],
                                        _ => panic!("No such input {}", io_id)
                                    };
                                    patch.add_connection(route, in_id);
                                }
                                while let Some(output) = route_el.take_child("output") {
                                    let m_id = output.attributes.get("module").unwrap();
                                    let _m_id = m_id.parse::<u16>().unwrap();
                                    
                                    let io_id = output.attributes.get("channel").unwrap();
                                    let _io_id = io_id.parse::<usize>().unwrap() - 1;

                                    let op_id = operators.get(&_m_id).unwrap();

                                    let out_id = match &patch[*op_id] {
                                        Module::Operator(_, _, outs) => outs[_io_id],
                                        _ => panic!("No such output {}", io_id)
                                    };
                                    patch.add_connection(out_id ,route);
                                }
                            }
                        }
                        name @ _ => { eprintln!("Unimplemented module {:?}", name)}
                    }
                }
                eprintln!("Project Loaded");
                eprintln!("{} Nodes", patch.node_count());
                eprintln!("{} Edges", patch.connection_count());
                let root = patch.add_node(Module::Master);
                patch.set_master(Some(root));
                patch.add_connection(*routes.get(&0).unwrap(), root);
            },
            _ => { eprintln!("unimplemented action {:?}", a); }
        }
    })
}
