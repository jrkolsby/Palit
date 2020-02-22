use std::fs::File;
use std::io::Write;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::collections::{HashMap, LinkedList};
use std::thread;
use std::time;
use std::sync::{Arc, RwLock, atomic::Ordering, atomic::AtomicU32};
use std::ops::{Deref, DerefMut};
use sample::{signal, Signal, Sample, Frame, ring_buffer};
use sample::interpolate::{Converter, Floor, Linear, Sinc};
use xmltree::Element;
use hound;
use object_pool::Pool;
use chrono::prelude::*;
use libcommon::{Action, Offset, Note, Key, Param, param_map, param_add, mark_map, mark_add};

use crate::core::{SAMPLE_HZ, BUF_SIZE, CHANNELS, BIT_RATE};
use crate::core::{SF, Output};

const SCRUB_MAX: f64 = 0.25;
const SCRUB_ACC: f64 = 0.01;

pub struct AudioRegion {
    pub id: u16,
    pub buffer: Vec<Vec<[Output; CHANNELS]>>,
    pub offset: Offset,
    pub duration: Offset,
    pub asset_in: Offset,
    pub gain: f32,
    pub asset_id: u16,
    pub asset_src: String,
}

pub struct MidiRegion {
    pub id: u16,
    pub notes: Vec<Note>,
    pub note_queue: Vec<Note>,
    pub offset: Offset,
    pub duration: Offset,
}

pub struct Store {
    pub bpm: u16,
    pub meter_beat: u16,
    pub meter_note: u16,
    pub loop_on: bool,
    pub loop_in: Offset,
    pub loop_out: Offset,
    pub duration: Offset,
    pub playhead: Offset, 
    pub audio_regions: Vec<AudioRegion>,
    pub midi_regions: Vec<MidiRegion>,
    pub velocity: f64,
    pub scrub: Option<bool>,
    pub scrub_max: f64,
    pub recording: u8,
    pub monitor: bool,
    pub mute: bool,
    pub solo: bool,
    pub track_id: u16,
    pub out_queue: Vec<Action>,
    pub sample_rate: u32,
    pub beat: Offset,
    pub zoom: Offset,
    pub pool: Option<Pool<'static, Vec<[Output; CHANNELS]>>>,
    pub writer: Option<thread::JoinHandle<()>>,
    pub rec_region: Arc<RwLock<Option<AudioRegion>>>,
    pub written: Arc<AtomicU32>, 
    pub rec_region_midi: Option<usize>,
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
        scrub_max: 0.25,
        monitor: true,
        mute: false,
        solo: false,
        recording: 0,
        audio_regions: vec![],
        midi_regions: vec![],
        track_id: 0,
        out_queue: vec![],
        sample_rate: SAMPLE_HZ as u32,
        beat: 0,
        zoom: 1,
        // Make SURE not to clone these when implementing undo/redo
        pool: None,
        writer: None,
        rec_region: Arc::new(RwLock::new(None)),
        written: Arc::new(AtomicU32::new(0)),
        rec_region_midi: None,
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
    match store.recording { 
        2 => {
            let region_count = store.written.load(Ordering::SeqCst);
            let region_guard = store.rec_region.write();
            match region_guard {
                Ok(mut option_region) => {
                    match option_region.deref_mut() {
                        Some(ref mut _region) => {
                            // We are stopped and the worker is finished writing
                            if region_count as usize % BUF_SIZE == 0 {
                                client_actions.push(Action::AddRegion(
                                    store.track_id, 
                                    _region.id, 
                                    _region.asset_id,
                                    _region.offset, 
                                    _region.duration, 
                                    _region.asset_in,
                                    _region.asset_src.clone()
                                ));
                            }
                            if store.velocity == 0.0 && region_count == _region.duration {
                                // Make a new guard to get rid of the region
                                let src = _region.asset_src.to_owned();
                                client_actions.push(Action::AddRegion(
                                    store.track_id, 
                                    _region.id, 
                                    _region.asset_id, 
                                    _region.offset, 
                                    _region.duration, 
                                    _region.asset_in,
                                    _region.asset_src.clone()
                                ));
                                store.audio_regions.push(AudioRegion {
                                    id: _region.id,
                                    offset: _region.offset,
                                    buffer: _region.buffer.to_owned(),
                                    asset_id: _region.asset_id,
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
        },
        _ => {}
    }

    for a in store.out_queue.iter() {
        match a {
            Action::AddMidiRegion(_, _, _, _) |
            Action::AddRegion(_, _, _, _, _, _, _) |
            Action::AddNote(_,_) |
            Action::Goto(_) |
            Action::Tick => {
                client_actions.push(a.clone());
            },
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
fn frame_with_offset(region: &AudioRegion, offset: usize) -> [Output; CHANNELS] {
    let index = offset / BUF_SIZE;
    return region.buffer[index][offset - (BUF_SIZE * index)];
}

// Worker thread for writing to disk during record
fn write_recording_region(source_region: Arc<RwLock<Option<AudioRegion>>>, source_count: Arc<AtomicU32>) {
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
                                if count % BUF_SIZE as u32 == 0 {
                                    _writer.flush();
                                }
                                source_count.store(count, Ordering::SeqCst);
                            }
                        }
                    },
                    None => { 
                        if let Some(mut _writer) = writer.take() {
                            _writer.flush();
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
        Action::LoopMode(on) => {
            store.loop_on = on;
        },
        Action::SetLoop(l_in, l_out) => {
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
        Action::Scrub(dir) => {
            if let Some(current_dir) = store.scrub {
                if current_dir == dir {
                    store.scrub_max *= 2.0;
                } else {
                    store.scrub_max = SCRUB_MAX;
                }
            }
            store.scrub = Some(dir);
        },
        Action::Play => {
            store.velocity = 1.0; 
            store.scrub = None;
            store.scrub_max = SCRUB_MAX;
        },
        Action::Record => { 
            store.velocity = 1.0; 
            store.scrub = None;
            store.scrub_max = SCRUB_MAX;
            let mut new_region_id = store.midi_regions.iter().fold(0, |max, r| 
                if r.id > max { r.id } else {max}) + 1;
            new_region_id = store.audio_regions.iter().fold(new_region_id, |max, r| 
                if r.id > max {r.id} else {max}) + 1;
            match store.recording { 
                2 => {
                    let mut new_asset_id = store.audio_regions.iter().fold(0, |max, r| 
                        if r.asset_id > max {r.asset_id} else {max}) + 1;
                    let timestamp = chrono::offset::Local::now().format("%s").to_string();
                    let new_src = format!("/usr/local/palit/assets/{}_{}.wav", 
                        timestamp, store.track_id);
                    let mut region_guard = store.rec_region.write().unwrap();
                    *region_guard = Some(AudioRegion {
                        id: new_region_id,
                        offset: store.playhead,
                        buffer: vec![],
                        duration: 0,
                        asset_in: 0,
                        gain: 1.0,
                        asset_id: new_asset_id,
                        asset_src: new_src.clone(),
                    });
                    store.out_queue.push(Action::AddRegion(
                        store.track_id, 
                        new_region_id, 
                        new_asset_id,
                        store.playhead, 
                        0, 
                        0,
                        new_src,
                    ));
                },
                1 => {
                    let new_region = MidiRegion {
                        id: new_region_id,
                        notes: vec![],
                        note_queue: vec![],
                        duration: 0,
                        offset: store.playhead,
                    };
                    store.rec_region_midi = Some(store.midi_regions.len());
                    store.midi_regions.push(new_region);
                    store.out_queue.push(Action::AddMidiRegion(
                        store.track_id, 
                        new_region_id, 
                        store.playhead, 
                        0, 
                    ));
                },
                _ => {}
            }
        },
        Action::Stop => { 
            store.velocity = 0.0; 
            store.scrub = None;
            if let Some(index) = &store.rec_region_midi {
                let midi_region = &store.midi_regions[*index];
                store.out_queue.push(Action::AddMidiRegion(
                    store.track_id, 
                    midi_region.id, 
                    midi_region.offset, 
                    midi_region.duration, 
                ));
                store.rec_region_midi = None;
            }
            if store.loop_on {
                store.playhead = store.loop_in;
            }
        },
        Action::RecordTrack(t_id, mode) => { 
            if store.track_id == t_id {
                store.recording = mode;
                match mode {
                    // Mode 2 (AUDIO)
                    2 => {
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
                    // Mode 0 (OFF) or 1 (MIDI)
                    _ => {
                        store.pool = None;
                        store.writer = None;
                        store.written.store(0, Ordering::SeqCst);
                    },
                }
            }
        },
        Action::MuteTrack(t_id, is_on) => { 
            if store.track_id == t_id {
                store.mute = is_on;
            }
        },
        Action::MonitorTrack(t_id, is_on) => { 
            if store.track_id == t_id {
                store.monitor = is_on; 
            }
        },
        Action::Loop(loop_in, loop_out) => { 
            store.loop_in = loop_in; 
            store.loop_out = loop_out; 
            store.loop_on = true;
        },
        Action::LoopOff => { store.loop_on = false; },
        Action::Goto(offset) => { 
            store.playhead = offset; 
        },
        Action::NoteOn(note, vel) => {
            // Push a new note to the end of store.notes 
            // ... and redistribute the t_in and t_out 
            // ... based on the rate and samples per bar
            if let Some(index) = store.rec_region_midi {
                let mut midi_region = &mut store.midi_regions[index];
                midi_region.note_queue.push(Note {
                    id: (midi_region.notes.len() + midi_region.note_queue.len()) as u16,
                    r_id: midi_region.id,
                    t_in: store.playhead,
                    t_out: 0,
                    note, 
                    vel,
                });
                // We need to expand the midi region unless notes will
                // ... not be played which were recorded during looping
                if midi_region.offset > store.playhead {
                    midi_region.offset = store.playhead;
                }
            }
            if store.monitor {
                store.out_queue.push(Action::NoteOn(note, vel));
            }
        },
        Action::NoteOff(note) => {
            if let Some(index) = store.rec_region_midi {
                let mut midi_region = &mut store.midi_regions[index];
                if let Some(on_index) = midi_region.note_queue.iter().position(|n| n.note == note) {
                        let on_note = midi_region.note_queue.remove(on_index);
                        let recorded_note = Note {
                            id: on_note.id,
                            r_id: on_note.r_id,
                            t_in: on_note.t_in,
                            t_out: store.playhead,
                            note: on_note.note,
                            vel: on_note.vel,
                        };
                        store.out_queue.push(Action::AddNote(
                            store.track_id,
                            recorded_note.clone(),
                        ));
                        midi_region.notes.push(recorded_note);
                        if midi_region.duration < store.playhead - midi_region.offset {
                            midi_region.duration = store.playhead - midi_region.offset;
                        }
                }
            }
            if store.monitor {
                store.out_queue.push(Action::NoteOff(note));
            }
        },
        Action::Zoom(size) => {
            store.zoom = if size >= 1 { size as Offset } else { 1 };
        },
        Action::MoveRegion(t_id, r_id, offset) => {
            if store.track_id == t_id {
                if let Some(mut region) = store.midi_regions.iter_mut().find(|r| r.id == r_id) {
                    let diff: i32 = offset as i32 - region.offset as i32;
                    for note in region.notes.iter_mut() {
                        note.t_in = (note.t_in as i32 + diff) as Offset;
                        note.t_out = (note.t_out as i32 + diff) as Offset;
                    }
                    region.offset = offset;
                }
                if let Some(mut region) = store.audio_regions.iter_mut().find(|r| r.id == r_id) {
                    region.offset = offset;
                }
            }
        },
        Action::DelRegion(t_id, r_id) => {
            if store.track_id == t_id {
                store.audio_regions.retain(|r| r.id != r_id);
                store.midi_regions.retain(|r| r.id != r_id);
            }
        },
        Action::SplitRegion(t_id, r_id, offset) => {
            if store.track_id == t_id {
                let mut new_region_id = store.midi_regions.iter().fold(0, |max, r| 
                    if r.id > max { r.id } else {max}) + 1;
                new_region_id = store.audio_regions.iter().fold(new_region_id, |max, r| 
                    if r.id > max {r.id} else {max}) + 1;
                if let Some(first_region) = store.midi_regions.iter_mut().find(|r| r.id == r_id) {
                    let mut second_region = MidiRegion {
                        id: new_region_id,
                        notes: vec![],
                        note_queue: vec![],
                        offset, // Starts at split, duration of original out offset - split
                        duration: first_region.offset + first_region.duration - offset,
                    };
                    first_region.duration = offset - first_region.offset;
                    for note in first_region.notes.iter_mut() {
                        // Copy notes after the split to second_region
                        if note.t_in >= offset {
                            second_region.notes.push(Note {
                                id: note.id,
                                r_id: second_region.id,
                                t_in: note.t_in,
                                t_out: note.t_out,
                                note: note.note, 
                                vel: note.vel,
                            });
                        }
                        // Clip note endings which overlap the split
                        if note.t_in < offset && note.t_out >= offset {
                            note.t_out = offset;
                        }
                    }
                    // Only keep notes before the split in first_region
                    first_region.notes.retain(|n| n.t_in < offset);
                    store.out_queue.push(Action::AddMidiRegion(
                        store.track_id, 
                        first_region.id, 
                        first_region.offset, 
                        first_region.duration, 
                    ));
                    for note in first_region.notes.iter() {
                        store.out_queue.push(Action::AddNote(store.track_id, note.clone()))
                    }
                    store.out_queue.push(Action::AddMidiRegion(
                        store.track_id, 
                        second_region.id, 
                        second_region.offset, 
                        second_region.duration, 
                    ));
                    for note in second_region.notes.iter() {
                        store.out_queue.push(Action::AddNote(store.track_id, note.clone()))
                    }
                    store.midi_regions.push(second_region);
                }
                if let Some(mut first_region) = store.audio_regions.iter_mut().find(|r| r.id == r_id) {
                    let mut second_region = AudioRegion {
                        id: new_region_id,
                        offset,
                        duration: first_region.offset + first_region.duration - offset,
                        asset_id: first_region.asset_id,
                        asset_in: first_region.asset_in + offset - first_region.offset,
                        asset_src: first_region.asset_src.clone(),
                        gain: first_region.gain,
                        buffer: first_region.buffer.clone(),
                    };
                    first_region.duration = offset - first_region.offset;
                    store.out_queue.push(Action::AddRegion(
                        store.track_id, 
                        second_region.id, 
                        second_region.asset_id, 
                        second_region.offset, 
                        second_region.duration, 
                        second_region.asset_in, 
                        second_region.asset_src.clone()
                    ));
                    store.out_queue.push(Action::AddRegion(
                        store.track_id, 
                        first_region.id, 
                        first_region.asset_id, 
                        first_region.offset, 
                        first_region.duration,
                        first_region.asset_in, 
                        first_region.asset_src.clone()
                    ));
                    store.audio_regions.push(second_region);
                    // Clip buffer at split and create new region and buffer for trimmings
                }
            }
        },
        Action::LoopRegion(t_id, r_id) => {},
        _ => {}
    }
}

pub fn compute_buf(store: &mut Store, buffer: &mut [[Output; CHANNELS]]) {
    // Exponential velocity scrub (tape inertia)
    let playback_rate = store.velocity.abs();
    if let Some(dir) = store.scrub {
        let expo_vel = store.velocity + if dir { SCRUB_ACC } 
            else { -SCRUB_ACC };
        store.velocity = if expo_vel > store.scrub_max { store.scrub_max } 
            else if expo_vel < -store.scrub_max { -store.scrub_max } 
            else { expo_vel }
    }
    if playback_rate == 0.0 { 
        dsp::slice::map_in_place(buffer, |a| { if store.monitor { a } else { [0.0, 0.0] } });
    } else if playback_rate == 1.0 && store.scrub.is_none() {
        // Audio Record Mode
        if store.recording == 2 {
            let (this_pool, this_region) = (
                store.pool.as_mut().unwrap(),
                store.rec_region.clone()
            );
            // option_region is of type RwLockGuard<Option<Region>>
            let region_guard = this_region.write();
            match region_guard {
                Ok(mut option_region) => {
                    // When a RwLockGuard<> contains an optional, you can't do:
                    // if let Some(x) = guard { ... }
                    // but you can call methods on its inner object, so instead:
                    match option_region.deref_mut() {
                        Some(_region) => {
                            for (i, frame) in buffer.iter().enumerate() {
                                let index = _region.duration as usize % BUF_SIZE;
                                if index == 0 {
                                    if let Some(new_buf) = this_pool.try_pull() {
                                        _region.buffer.push(new_buf.to_vec());
                                    } else {
                                        // Out of space! Stop record
                                        store.recording = 0;
                                        break;
                                    }
                                }
                                _region.buffer.last_mut().unwrap()[index] = *frame;
                                _region.duration += 1;
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        } 
        dsp::slice::map_in_place(buffer, |a| {
            let frame = compute(store);
            [
                frame[0] + if store.monitor { a[0] } else { 0.0 },
                frame[1] + if store.monitor { a[1] } else { 0.0 }
            ] 
        });
    } else {
        let thru = store.monitor;
        let mut source = signal::gen_mut(|| compute(store) );
        /*
        let frames = ring_buffer::Fixed::from(vec![[0.0]; 100]);
        let interp = Sinc::new(frames);
        let mut resampled = source.from_hz_to_hz(interp, SAMPLE_HZ, SAMPLE_HZ * playback_rate);
        */
        let interp = Linear::from_source(&mut source);
        let mut resampled = source.scale_hz(interp, playback_rate);
        dsp::slice::map_in_place(buffer, |a| {
            let frame = resampled.next();
            [
                frame[0] + if thru { a[0] } else { 0.0 },
                frame[1] + if thru { a[1] } else { 0.0 }
            ] 
        });
    }

}

pub fn compute(store: &mut Store) -> [Output; CHANNELS] {
    let mut z: [Output; CHANNELS] = [0.0, 0.0];
    if store.velocity == 0.0 { return z; }
    for region in store.audio_regions.iter() {
        if store.playhead >= region.offset && 
            store.playhead - region.offset < region.duration {
            let offset = (store.playhead - region.offset + region.asset_in) as usize;
            let x = frame_with_offset(&region, offset);
            z = [x[0] * region.gain, x[1] * region.gain];
        }
    }
    for region in store.midi_regions.iter() {
        if store.playhead >= region.offset && 
            store.playhead <= region.offset + region.duration {
            for note in region.notes.iter() {
                if (store.velocity > 0.0 && note.t_in == store.playhead) ||
                    (store.velocity < 0.0 && note.t_out == store.playhead) {
                    store.out_queue.push(Action::NoteOn(note.note, note.vel));
                }
                if (store.velocity > 0.0 && note.t_out == store.playhead) ||
                    (store.velocity < 0.0 && note.t_in == store.playhead) {
                    store.out_queue.push(Action::NoteOff(note.note));
                }
            }
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

    // Only emit timing events from one track
    if store.track_id == 1 {
        // Metronome
        if store.playhead % store.beat == 0 && store.track_id == 1 {
            store.out_queue.push(Action::Tick);
        }
        // Client Playhead
        if store.playhead % (store.beat / store.zoom) == 0 {
            store.out_queue.push(Action::Goto(store.playhead));
        }

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
    for region in s.audio_regions.iter() {
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

    store.bpm = (*params.get("bpm").unwrap_or(&127.0)) as u16;
    store.meter_beat = (*params.get("meter_beat").unwrap_or(&4.0)) as u16;
    store.meter_note = (*params.get("meter_note").unwrap_or(&4.0)) as u16;
    store.loop_in = (*marks.get("loop_in").unwrap_or(&0)).try_into().unwrap();
    store.loop_out = (*marks.get("loop_out").unwrap_or(&0)).try_into().unwrap();
    store.duration = (*marks.get("seq_out").unwrap_or(&48000) - 
                      *marks.get("seq_in").unwrap_or(&0)).try_into().unwrap();
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
            let duration: &str = region.attributes.get("duration").unwrap();
            let a_in: &str = region.attributes.get("in").unwrap();

            let _r_id: u16 = r_id.parse().unwrap();
            let _a_id: u16 = a_id.parse().unwrap();
            let _a_in: Offset = a_in.parse().unwrap();
            let _duration: Offset = duration.parse().unwrap();
            let _offset: Offset = offset.parse().unwrap();

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

            store.audio_regions.push(AudioRegion {
                id: _r_id,
                asset_id: _a_id,
                asset_in: _a_in,
                offset: _offset,
                duration: _duration,
                asset_src: _src.unwrap(),
                gain: 1.0,
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