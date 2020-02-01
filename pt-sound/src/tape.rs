use std::fs::File;
use std::io::Write;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::collections::{HashMap, LinkedList};
use std::thread;
use std::time;
use std::sync::{Arc, RwLock, atomic::Ordering, atomic::AtomicU32};
use std::ops::{Deref, DerefMut};

use sample::{signal, Signal, Sample, Frame};
use xmltree::Element;
use hound;
use object_pool::Pool;
use chrono::prelude::*;

use crate::core::{SAMPLE_HZ, BUF_SIZE, CHANNELS, BIT_RATE};
use crate::core::{SF, SigGen, Output, Note, Key, Offset};
use crate::action::Action;
use crate::document::{param_map, param_add, mark_map, mark_add};

pub struct Region {
    pub id: u16,
    pub buffer: Vec<Vec<[Output; CHANNELS]>>,
    pub offset: Offset,
    pub duration: Offset,
    pub asset_in: Offset,
    pub asset_out: Offset,
    pub gain: f32,
    pub asset_id: u16,
    pub asset_src: String,
}

pub struct Store {
    pub bpm: u16,
    pub meter_beat: u16,
    pub meter_note: u16,
    pub loop_on: bool,
    pub loop_in: u32,
    pub loop_out: u32,
    pub duration: u32,
    pub playhead: u32, 
    pub regions: Vec<Region>,
    pub notes: Vec<Note>,
    pub velocity: f64,
    pub scrub: Option<bool>,
    pub recording: u8,
    pub monitor: bool,
    pub mute: bool,
    pub solo: bool,
    pub track_id: u16,
    pub out_queue: Vec<Action>,
    pub note_queue: Vec<Note>,
    pub sample_rate: u32,
    pub beat: u32,
    pub pool: Option<Pool<'static, Vec<[Output; CHANNELS]>>>,
    pub writer: Option<thread::JoinHandle<()>>,
    pub rec_region: Arc<RwLock<Option<Region>>>,
    pub written: Arc<AtomicU32>, 
}

fn calculate_beat(sample_rate: u32, bpm: u16) -> u32 {
    (60 * sample_rate) / (bpm as u32)
}

pub fn init() -> Store {
    return Store {
        bpm: 127,
        duration: 960000,
        meter_beat: 4,
        meter_note: 4,
        loop_on: false,
        loop_in: 0,
        loop_out: 0,
        playhead: 0,
        velocity: 0.0,
        scrub: None,
        monitor: true,
        mute: false,
        solo: false,
        recording: 0,
        regions: vec![],
        notes: vec![],
        track_id: 0,
        out_queue: vec![],
        note_queue: vec![],
        sample_rate: SAMPLE_HZ as u32,
        beat: 0,
        // Make SURE not to clone these when implementing undo/redo
        pool: None,
        writer: None,
        rec_region: Arc::new(RwLock::new(None)),
        written: Arc::new(AtomicU32::new(0)),
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>,
        Option<Vec<Action>>,
        Option<Vec<Action>>) {
    let mut client_actions = vec![];
    let mut output_actions = vec![];

    // Check if we can join the writer and 
    // ... finalize our recorded region :D
    if store.recording == 1 {
        let region_count = store.written.load(Ordering::SeqCst);
        let region_guard = store.rec_region.write();
        match region_guard {
            Ok(mut option_region) => {
                match option_region.deref_mut() {
                    Some(ref mut _region) => {
                        // We are stopped and the worker is finished writing
                        if region_count as usize % BUF_SIZE == 0 {
                            client_actions.push(Action::AddRegion(0,
                                store.track_id, 
                                _region.id, 
                                _region.asset_id,
                                _region.offset, 
                                _region.duration, 
                                _region.asset_src.clone()
                            ));
                        }
                        if store.velocity == 0.0 && region_count == _region.duration {
                            // Make a new guard to get rid of the region
                            let src = _region.asset_src.to_owned();
                            // Ideally we would do this in the worker...
                            /*
                            let writer = hound::WavWriter::append(src.clone()).unwrap();
                            writer.finalize();
                            */
                            client_actions.push(Action::AddRegion(0,
                                store.track_id, 
                                _region.id, 
                                _region.asset_id, 
                                _region.offset, 
                                _region.duration, 
                                _region.asset_src.clone()
                            ));
                            store.regions.push(Region {
                                id: _region.id,
                                offset: _region.offset,
                                buffer: _region.buffer.to_owned(),
                                asset_id: _region.asset_id,
                                asset_out: _region.asset_out,
                                asset_in: _region.asset_in,
                                asset_src: src,
                                duration: _region.duration,
                                gain: _region.gain,
                            });
                            *option_region = None;
                            store.written.store(0, Ordering::SeqCst);
                        }
                    },
                    None => {}
                }
            },
            Err(_) => {}
        }
    }

    for a in store.out_queue.iter() {
        match a {
            Action::AddRegion(_, _, _, _, _, _, _) |
            Action::AddNote(_, _) |
            Action::Tick(_) => client_actions.push(a.clone()),
            _ => output_actions.push(a.clone())
        }
    }

    store.out_queue.clear();

    (
        if output_actions.len() > 0 { Some(output_actions) } else { None }, 
        None, 
        if client_actions.len() > 0 { Some(client_actions) } else { None }
    )
}

// Indexes via an offset into the two dimensional region buffer 
fn frame_with_offset(region: &Region, offset: usize) -> [Output; CHANNELS] {
    let index = offset / BUF_SIZE;
    return region.buffer[index][offset - (BUF_SIZE * index)];
}

fn write_recording_region(source_region: Arc<RwLock<Option<Region>>>, source_count: Arc<AtomicU32>) {
    let wav_spec = hound::WavSpec {
        channels: CHANNELS as u16,
        sample_rate: SAMPLE_HZ as u32,
        bits_per_sample: BIT_RATE as u16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer: Option<hound::WavWriter<std::io::BufWriter<File>>> = None;
    loop {
        let region_guard = source_region.read();
        match region_guard {
            Ok(option_region) => {
                match option_region.deref() {
                    Some(_region) => {
                        let mut count = source_count.load(Ordering::SeqCst);
                        match writer {
                            None => writer = Some(
                                hound::WavWriter::create(_region.asset_src.clone(), wav_spec).unwrap()
                            ),
                            Some(ref mut _writer) => {
                                while count < _region.duration {
                                    let frame = frame_with_offset(&_region, count as usize);
                                    // Needed to convert to integer wave file
                                    _writer.write_sample((frame[0] * std::i16::MAX as f32) as i16);
                                    _writer.write_sample((frame[1] * std::i16::MAX as f32) as i16);
                                    count += 1;
                                }
                                _writer.flush();
                                source_count.store(count, Ordering::SeqCst);
                            }
                        }
                    },
                    None => { 
                        if let Some(_writer) = writer.take() {
                            _writer.finalize();
                            writer = None;
                        }
                    }
                }
            },
            Err(e) => {}
        }
        // Don't hog the CPU!
        thread::sleep(time::Duration::from_millis(3)); 
    }
}

pub fn dispatch(store: &mut Store, a: Action) {
    match a {
        Action::LoopMode(_, on) => {
            store.loop_on = on;
        },
        Action::SetLoop(_, l_in, l_out) => {
            store.loop_in = l_in;
            store.loop_out = l_out;
        },
        Action::SetTempo(t) => {
            store.bpm = t;
            store.beat = calculate_beat(store.sample_rate, t);
        },
        Action::SetMeter(beat, note) => {
            store.meter_beat = beat;
            store.meter_note = note;
        },
        Action::Scrub(_, dir) => {
            store.scrub = Some(dir);
        },
        Action::Play(_) => { 
            store.velocity = 1.0; 
            store.scrub = None;
            if store.recording == 1 {
                let mut new_asset_id = store.regions.iter().fold(0, |max, r| 
                    if r.asset_id > max {r.asset_id} else {max}) + 1;
                let mut new_region_id = store.regions.iter().fold(0, |max, r| 
                    if r.id > max {r.id} else {max}) + 1;
                let timestamp = chrono::offset::Local::now().format("%s").to_string();
                let new_src = format!("/usr/local/palit/assets/{}_{}.wav", 
                    timestamp, store.track_id);
                let mut region_guard = store.rec_region.write().unwrap();
                *region_guard = Some(Region {
                    id: new_region_id,
                    offset: store.playhead,
                    buffer: vec![],
                    duration: 0,
                    asset_in: 0,
                    asset_out: 0,
                    gain: 1.0,
                    asset_id: new_asset_id,
                    asset_src: new_src.clone(),
                });
                store.out_queue.push(Action::AddRegion(0,
                    store.track_id, 
                    new_region_id, 
                    new_asset_id,
                    store.playhead, 
                    0, 
                    new_src,
                ));
            }
        },
        Action::Stop(_) => { 
            store.velocity = 0.0; 
            store.scrub = None;
            if store.loop_on {
                store.playhead = store.loop_in;
            }
        },
        Action::RecordAt(_, t_id, mode) => { 
            if store.track_id == t_id {
                store.recording = mode;
                match mode {
                    1 => {
                        if store.pool.is_none() {
                            store.pool = Some(Pool::new(30, || vec![[0.0; CHANNELS]; BUF_SIZE]));
                        }
                        if store.writer.is_none() {
                            // https://stackoverflow.com/questions/42043823/design-help-threading-within-a-struct
                            // Doesn't actually clone, just increments the reference counter
                            let source_region = store.rec_region.clone();
                            let source_count = store.written.clone();
                            store.writer = Some(thread::spawn(|| write_recording_region(source_region, source_count)));
                        }
                    },
                    // Mode 0 (OFF) or 2 (MIDI)
                    _ => {
                        store.pool = None;
                        /*
                        // take() vs to_owned() ?
                        if let Some(_writer) = store.writer.take() {
                            _writer.join();
                        }
                        */
                        store.writer = None;
                        store.written.store(0, Ordering::SeqCst);
                    },
                }
            }
        },
        Action::MuteAt(_, t_id, is_on) => { 
            if store.track_id == t_id {
                store.mute = is_on;
            }
        },
        Action::MonitorAt(_, t_id, is_on) => { 
            if store.track_id == t_id {
                store.monitor = is_on; 
            }
        },
        Action::Loop(_, loop_in, loop_out) => { 
            store.loop_in = loop_in; 
            store.loop_out = loop_out; 
            store.loop_on = true;
        },
        Action::LoopOff(_) => { store.loop_on = false; },
        Action::Goto(_, offset) => { 
            store.note_queue.clear();
            store.playhead = offset; 
        },
        Action::NoteOn(note, vel) => {
            // Push a new note to the end of store.notes 
            // ... and redistribute the t_in and t_out 
            // ... based on the rate and samples per bar
            if store.recording == 2 && store.velocity > 0.0 {
                store.note_queue.push(Note {
                    id: store.notes.len() as u16,
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
                if store.recording == 2 && store.velocity != 0.0 {
                    let on_note = store.note_queue.remove(on_index);
                    let recorded_note = Note {
                        id: on_note.id,
                        t_in: on_note.t_in,
                        t_out: store.playhead,
                        note: on_note.note,
                        vel: on_note.vel,
                    };
                    store.out_queue.push(Action::AddNote(
                        recorded_note.id, recorded_note.clone(),
                    ));
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

pub fn compute(store: &mut Store) -> [Output; CHANNELS] {
    let mut z: [Output; CHANNELS] = [0.0, 0.0];
    if store.velocity == 0.0 { return z; }
    for region in store.regions.iter_mut() {
        if store.playhead >= region.offset && 
            store.playhead - region.offset < region.duration {
            let offset = (store.playhead - region.offset) as usize;
            let x = frame_with_offset(&region, offset);
            z = [x[0] * region.gain, x[1] * region.gain];
        }
    }
    for note in store.notes.iter_mut() {
        if note.t_in == store.playhead {
            store.out_queue.push(Action::NoteOn(note.note, note.vel));
        }
        if note.t_out == store.playhead {
            store.out_queue.push(Action::NoteOff(note.note));
        }
    }
    // Prevent speaker damage
    z = [
        z[0].min(0.999).max(-0.999),
        z[1].min(0.999).max(-0.999),
    ];

    if store.velocity < 0.0 && store.playhead == 0 { 
        store.scrub = None;
        store.velocity = 0.0 
    }

    // Metronome
    if store.playhead % store.beat == 0 && store.track_id == 1 {
        store.out_queue.push(Action::Tick(store.playhead));
    }

    // Play direction
    store.playhead = if store.velocity > 0.0 { store.playhead + 1 }
        else if store.playhead > 0 { store.playhead - 1 } 
        else { store.playhead };

    // Looping
    if store.loop_on {
        if store.velocity > 0.0 && store.playhead >= store.loop_out {
            store.playhead = store.loop_in;
        }
    }
    z
}

pub fn write(s: Store, doc: Option<Element>) -> Element {
    let mut el = Element::new("timeline");

    param_add(&mut el, s.bpm, "bpm".to_string());
    param_add(&mut el, s.meter_beat, "meter_beat".to_string());
    param_add(&mut el, s.meter_note, "meter_note".to_string());

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

    store.bpm = (*params.get("bpm").unwrap_or(&127)).try_into().unwrap();
    store.duration = (*marks.get("seq_out").unwrap_or(&48000) - 
                   *marks.get("seq_in").unwrap_or(&0)).try_into().unwrap();
    store.meter_beat = (*params.get("meter_beat").unwrap_or(&4)).try_into().unwrap();
    store.meter_note = (*params.get("meter_note").unwrap_or(&4)).try_into().unwrap();
    store.loop_in = (*marks.get("loop_in").unwrap_or(&0)).try_into().unwrap();
    store.loop_out = (*marks.get("loop_out").unwrap_or(&0)).try_into().unwrap();
    store.beat = calculate_beat(store.sample_rate, store.bpm);

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

            let _r_id: u16 = r_id.parse().unwrap();
            let _a_id: u16 = a_id.parse().unwrap();
            let _a_in: u32 = a_in.parse().unwrap();
            let _a_out: u32 = a_out.parse().unwrap();

            let mut buffer = vec![];
            let mut _src: Option<String> = None;
            for (id, asset) in assets.iter() {
                if (_a_id == *id) {
                    let src: &str = asset.attributes.get("src").unwrap();
                    _src = Some(src.to_string());
                    let duration: &str = asset.attributes.get("size").unwrap();
                    let _duration: usize = duration.parse().unwrap();

                    buffer = Vec::with_capacity(_duration / BUF_SIZE);
                    
                    let mut wav_f = hound::WavReader::open(src).unwrap();
                    let spec = wav_f.spec();
                    let channels = spec.channels as usize;
                    let bitrate = spec.bits_per_sample;

                    let mut push_sample = |i, sample| {
                        if i % (BUF_SIZE * channels) == 0 {
                            buffer.push(vec![])
                        }
                        let frame_index = i % channels;
                        if frame_index == 0 {
                            buffer.last_mut().unwrap().push([0.0; CHANNELS]);
                        }
                        buffer.last_mut().unwrap().last_mut().unwrap()[frame_index] = sample
                    };

                    if spec.sample_format == hound::SampleFormat::Float {
                        for (i, sample) in wav_f.samples::<f32>().enumerate() {
                            push_sample(i, sample.unwrap());
                        }
                    } else {
                        for (i, sample) in wav_f.samples::<i32>().enumerate() {
                            push_sample(i, sample.unwrap() as f32 / 2_f32.powf(bitrate as f32));
                        }
                    }
                }
            }

            store.regions.push(Region {
                id: _r_id,
                offset: offset.parse().unwrap(),
                gain: 1.0,
                duration: _a_out - _a_in,
                asset_id: _a_id,
                asset_in: _a_in,
                asset_out: _a_out,
                asset_src: _src.unwrap(),
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