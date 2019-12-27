use std::fs::File;
use std::io::Write;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::collections::HashMap;

use sample::{signal, Signal, Sample};
use xmltree::Element;
use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen, Output, Note, Key, Offset};
use crate::action::Action;
use crate::document::{param_map, param_add, mark_map, mark_add};

pub struct Region {
    pub buffer: Vec<Output>,
    pub offset: u32,
    pub duration: u32,
    pub asset_in: u32,
    pub asset_out: u32,
    pub gain: f32,
    pub asset_id: u16,
}

pub struct Store {
    pub bpm: u16,
    pub time_beat: usize,
    pub time_note: usize,
    pub loop_on: bool,
    pub loop_in: u32,
    pub loop_out: u32,
    pub duration: u32,
    pub playhead: u32, 
    pub regions: Vec<Region>,
    pub notes: Vec<Note>,
    pub playing: bool,
    pub recording: bool,
    pub monitor: bool,
    pub track_id: u16,
    pub out_queue: Vec<Action>,
    pub note_queue: Vec<Note>,
}

pub fn init() -> Store {
    return Store {
        bpm: 127,
        duration: 960000,
        time_beat: 4,
        time_note: 4,
        loop_on: false,
        loop_in: 0,
        loop_out: 0,
        playhead: 0,
        playing: false,
        monitor: true,
        recording: false,
        regions: vec![],
        notes: vec![],
        track_id: 0,
        out_queue: vec![],
        note_queue: vec![]
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>,
        Option<Vec<Action>>,
        Option<Vec<Action>>
    ) {
        let client_actions = if store.playing && store.playhead % 65536 == 0 {
            Some(vec![Action::Tick])
        } else { None };
        let output_actions = if store.out_queue.len() == 0 { None } else {
            let carry = store.out_queue.clone();
            store.out_queue.clear();
            Some(carry) 
        };
        (client_actions, None, output_actions) 
}

pub fn dispatch(store: &mut Store, a: Action) {
    match a {
        Action::Play(_) => { store.playing = true; },
        Action::Stop(_) => { 
            store.playing = false; 
            store.recording = false; 
            if store.loop_on {
                store.playhead = store.loop_in;
            }
        },
        Action::RecordAt(_, t_id) => { 
            if store.track_id == t_id {
                store.recording = !store.recording;
            }
        },
        Action::MuteAt(_, t_id) => { 
            if store.track_id == t_id {
                store.recording = !store.recording;
            }
        },
        Action::Monitor(_) => { store.monitor = !store.monitor; },
        Action::Loop(_, loop_in, loop_out) => { 
            store.loop_in = loop_in; 
            store.loop_out = loop_out; 
            store.loop_on = true;
        },
        Action::LoopOff(_) => { store.loop_on = false; },
        Action::Goto(_, offset) => { store.playhead = offset; },
        Action::NoteOn(note, vel) => {
            // Push a new note to the end of store.notes 
            // ... and redistribute the t_in and t_out 
            // ... based on the rate and samples per bar
            if store.recording {
                eprintln!("queued note {}", note);
                store.note_queue.push(Note {
                    t_in: store.playhead,
                    t_out: 0,
                    note, 
                    vel,
                });
            }
            if store.monitor {
                store.out_queue.push(Action::NoteOn(note, vel));
            }
        },
        Action::NoteOff(note) => {
            if let Some(on_index) = store.note_queue.iter().position(|n| n.note == note) {
                let on_note = store.note_queue.remove(on_index);
                let recorded_note = Note {
                    t_in: on_note.t_in,
                    t_out: store.playhead,
                    note: on_note.note,
                    vel: on_note.vel,
                };
                if store.recording && store.playing {
                    store.notes.push(recorded_note);
                }
            }
            if store.monitor {
                store.out_queue.push(Action::NoteOff(note));
            }
        },
        _ => {}
    }
}

pub fn compute(store: &mut Store) -> Output {
    let mut z: f32 = 0.0;
    if !store.playing { return z; }
    for region in store.regions.iter_mut() {
        if store.playhead >= region.offset && store.playhead - region.offset < region.duration {
            let index = (store.playhead - region.offset) as usize;
            let x: f32 = region.buffer[index];
            z += x * region.gain;
        }
    }
    for note in store.notes.iter_mut() {

    }
    let z = z.min(0.999).max(-0.999);
    store.playhead = if store.loop_on {
        if store.playhead == store.loop_out {
            store.loop_in
        } else {
            store.playhead + 1
        }
    } else { 
        store.playhead + 1 
    };
    z
}

pub fn write(s: Store, doc: Option<Element>) -> Element {
    let mut el = Element::new("timeline");

    param_add(&mut el, s.bpm, "bpm".to_string());
    param_add(&mut el, s.time_beat, "time_beat".to_string());
    param_add(&mut el, s.time_note, "time_note".to_string());

    mark_add(&mut el, s.loop_in, "loop_in".to_string());
    mark_add(&mut el, s.loop_out, "loop_out".to_string());
    mark_add(&mut el, s.duration, "seq_out".to_string());
    mark_add(&mut el, 0, "seq_in".to_string());

    let track = Element::new("track");
    for region in s.regions.iter() {
        let r = Element::new("region");
    }

    /*
    Element::new("asset")
    Element::new("track")
        Element::new("region")
        */
    el 
}

pub fn read(doc: &mut Element) -> Option<Store> {
    let mut store = init();

    let (mut doc, mut params) = param_map(doc);
    let (mut doc, mut marks) = mark_map(doc);

    store.bpm = (*params.get("bpm").unwrap()).try_into().unwrap();
    store.duration = (*marks.get("seq_out").unwrap() - 
                   *marks.get("seq_in").unwrap()).try_into().unwrap();
    store.time_beat = (*params.get("time_beat").unwrap()).try_into().unwrap();
    store.time_note = (*params.get("time_note").unwrap()).try_into().unwrap();
    store.loop_in = (*marks.get("loop_in").unwrap()).try_into().unwrap();
    store.loop_out = (*marks.get("loop_out").unwrap()).try_into().unwrap();

    for (name, value) in params.drain() {
        param_add(doc, value, name);
    }

    for (name, offset) in marks.drain() {
        mark_add(doc, offset, name);
    }

    let mut assets: HashMap<u16, Element> = HashMap::new();

    while let Some(asset) = doc.take_child("asset") {
        let a_id: &str = asset.attributes.get("id").unwrap();
        assets.insert(a_id.parse().unwrap(), asset);
    }

    // Only take one track 
    if let Some(mut track) = doc.take_child("track") {
        let t_id: &str = track.attributes.get("id").unwrap();
        let _t_id = t_id.parse::<u16>().unwrap();
        store.track_id = _t_id;

        while let Some(region) = track.take_child("region") {
            let r_id: &str = region.attributes.get("id").unwrap();
            let a_id: &str = region.attributes.get("asset").unwrap();
            let offset: &str = region.attributes.get("offset").unwrap();
            let a_in: &str = region.attributes.get("in").unwrap();
            let a_out: &str = region.attributes.get("out").unwrap();

            let _a_id: u16 = a_id.parse().unwrap();
            let _a_in: u32 = a_in.parse().unwrap();
            let _a_out: u32 = a_out.parse().unwrap();

            let mut buffer: Vec<f32> = vec![];

            for (id, asset) in assets.iter() {
                if (_a_id == *id) {
                    let src: &str = asset.attributes.get("src").unwrap();
                    let duration: &str = asset.attributes.get("size").unwrap();

                    let mut wav_f = WaveFile::open("Who.wav").unwrap();
                    let mut wav_iter = wav_f.iter();

                    buffer = wav_iter.map(|f| f[0] as f32 * 0.0000001).collect();
                }
            }

            store.regions.push(Region {
                offset: offset.parse().unwrap(),
                gain: 1.0,
                duration: _a_out - _a_in,
                asset_id: _a_id,
                asset_in: _a_in,
                asset_out: _a_out,
                buffer,
            });
        }
    } else {
        return None;
    }

    for (id, asset) in assets.drain() {
        doc.children.push(asset);
    }

    return Some(store);
}