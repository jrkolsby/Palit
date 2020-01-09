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
use crate::document::{Document, read_document, param_map};
use crate::action::Action;

const MASTER_ROUTE: u16 = 1;

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
        // n_id Node ID
        // r_id Route ID
        // a_id Anchor ID (Any input or output from a module)
        // op_id Module Operator ID (Dispatches to a cluster of nodes)
        // 

        eprintln!("ACTION {:?}", a);
        match a {
            Action::LoopMode(n_id, _) |
            Action::SetLoop(n_id, _, _) |
            Action::Scrub(n_id, _) |
            Action::Goto(n_id, _) |
            Action::RecordAt(n_id, _) |
            Action::MuteAt(n_id, _) |
            Action::NoteOnAt(n_id, _, _) | 
            Action::NoteOffAt(n_id, _) |
            Action::SetParam(n_id, _, _) |
            Action::Play(n_id) | 
            Action::Stop(n_id) => {
                if let Some(id) = operators.get(&n_id) {
                    patch[*id].dispatch(a)
                }
            },
            Action::SetMeter(_, _) |
            Action::SetTempo(_) => {
                for (_, node) in operators.iter() {
                    patch[*node].dispatch(a.clone())
                }
            },
            Action::NoteOn(_,_) | Action::NoteOff(_) | Action::Octave(_) => {
                if let Some(id) = operators.get(&104) {
                    patch[*id].dispatch(a)
                }
            },
            Action::MoveRegion(m_id, r_id, track, offset) => {
                if let Some(n_id) = operators.get(&m_id) {
                    if let Some(node) = patch.node_mut(*n_id) {
                        node.dispatch(a)
                    }
                }
            },
            Action::AddRoute(r_id) => {
                let route = patch.add_node(Module::Passthru(vec![]));
                routes.insert(r_id, route);
            },
            Action::PatchIn(n_id, a_id, r_id) => {
                if let Some(route) = routes.get(&r_id) {
                    match &patch[*operators.get(&n_id).unwrap()] {
                        Module::Operator(_, anchors) => {
                            patch.add_connection(*route, anchors[a_id]);
                        }
                        _ => {}
                    };
                }
            },
            Action::PatchOut(n_id, a_id, r_id) => {
                if let Some(route) = routes.get(&r_id) {
                    match &patch[*operators.get(&n_id).unwrap()] {
                        Module::Operator(_, anchors) => {
                            patch.add_connection(anchors[a_id], *route);
                        }
                        _ => {}
                    };
                }
            },
            Action::DeletePatch(n_id, a_id, input) => {
                match &patch[*operators.get(&n_id).unwrap()] {
                    Module::Operator(_, anchors) => {
                        let id = anchors[a_id].clone();
                        for (_, route) in routes.iter() {
                            let edge = if input {
                                patch.find_connection(*route, id)
                            } else {
                                patch.find_connection(id, *route)
                            };
                            if let Some(e) = edge {
                                patch.remove_edge(e);
                            }
                        }
                    }
                    _ => {}
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
                            let mut anchors: Vec<NodeIndex> = vec![];
                            // Mutate el by removing track elements until
                            // none are left
                            while let Some(store) = tape::read(el) {
                                let tape = patch.add_node(Module::Tape(store));
                                anchors.push(tape); // INPUT
                                anchors.push(tape); // OUTPUT
                            }
                            let operator = patch.add_node(Module::Operator(vec![], 
                                anchors.clone(), 
                            ));
                            // Because each track is stored as two anchors,
                            // ... we need to make sure there is only one edge
                            // ... to each track, otherwise actions will be 
                            // ... dispatched two times. :^)
                            for anchor in anchors.iter() {
                                if patch.find_connection(operator, *anchor).is_none() {
                                    patch.add_connection(operator, *anchor);
                                }
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
                                vec![instrument, instrument])
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
                                vec![inst, inst])
                            );
                            patch.add_connection(operator, inst);
                            operators.insert(*id, operator);
                        },
                        "chord" => {
                            let store = chord::read(el).unwrap();
                            let inst = patch.add_node(Module::Chord(store));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![inst, inst])
                            );
                            patch.add_connection(operator, inst);
                            operators.insert(*id, operator);
                        },
                        "keyboard" => {
                            let (_, params) = param_map(el);
                            let shift = *params.get("octave").unwrap_or(&0) as Key;
                            let octave = patch.add_node(Module::Octave(vec![], shift));
                            //let shift = patch.add_node(Module::Octave(vec![], 4));
                            let operator = patch.add_node(Module::Operator(vec![], 
                                vec![octave, octave])
                            );
                            patch.add_connection(operator, octave);
                            operators.insert(*id, operator);
                        },
                        // This module should always be last in doc.modules or else 
                        // operators and routes maps won't be completely filled
                        "patch" => {
                            while let Some(mut route_el) = el.take_child("route") {
                                let r_id = route_el.attributes.get("id").unwrap();
                                let _r_id = r_id.parse::<u16>().unwrap();
                                let route = patch.add_node(Module::Passthru(vec![]));
                                routes.insert(_r_id, route);
                                while let Some(input) = route_el.take_child("input") {
                                    let m_id = input.attributes.get("module").unwrap();
                                    let _m_id = m_id.parse::<u16>().unwrap();

                                    let io_id = input.attributes.get("index").unwrap();
                                    let _io_id = io_id.parse::<usize>().unwrap();
                                    
                                    let op_id = operators.get(&_m_id).unwrap();

                                    let in_id = match &patch[*op_id] {
                                        Module::Operator(_, anchors) => anchors[_io_id],
                                        _ => panic!("No such input {}", io_id)
                                    };
                                    patch.add_connection(route, in_id);
                                }
                                while let Some(output) = route_el.take_child("output") {
                                    let m_id = output.attributes.get("module").unwrap();
                                    let _m_id = m_id.parse::<u16>().unwrap();
                                    
                                    let io_id = output.attributes.get("index").unwrap();
                                    let _io_id = io_id.parse::<usize>().unwrap();

                                    let op_id = operators.get(&_m_id).unwrap();

                                    let out_id = match &patch[*op_id] {
                                        Module::Operator(_, anchors) => anchors[_io_id],
                                        _ => panic!("No such output {}", io_id)
                                    };
                                    patch.add_connection(out_id ,route);
                                }
                            }
                        }
                        name @ _ => { eprintln!("Unimplemented module {:?}", name)}
                    }
                }
                let root = patch.add_node(Module::Master);
                patch.set_master(Some(root));
                patch.add_connection(*routes.get(&MASTER_ROUTE).unwrap(), root);
                eprintln!("Loaded {} Nodes", patch.node_count());
                eprintln!("Loaded {} Edges", patch.connection_count());
            },
            _ => { eprintln!("unimplemented action {:?}", a); }
        }
    })
}
