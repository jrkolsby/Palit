use std::borrow::BorrowMut;
use xmltree::Element;
use libcommon::{Action, Note, Key, Offset, Param, param_map};

pub struct Store {
    timer: Offset,
    length: Param,
    pattern: usize,
    bpm: Param,
    sample_rate: Offset,
    bar: Offset,
    notes: Vec<Note>,
    queue: Vec<Action>,
}

pub fn init() -> Store {
    Store {
        timer: 0,
        length: 4.0, // beats per loop
        pattern: 0,
        bpm: 127.0,
        sample_rate: 48000,
        bar: calculate_beat(48000, 127.0, 4.0),
        notes: vec![],
        queue: vec![],
    }
}

fn calculate_beat(sample_rate: Offset, bpm: Param, length: Param) -> Offset {
    ((sample_rate as Param * 60.0 / bpm as Param) * length) as Offset
}

fn distribute_notes(notes: &mut Vec<Note>, length: Offset, pattern: usize) {
    let len_i = notes.len() as f32;
    // given 3 notes in arpeggio, evenly space 6 NoteOn / NoteOff
    // actions, starting with the first event at timer == 0
    // |O  F  O  F  O  F  |
    let samples_per_event = length as f32 / (len_i * 2.0);
    match pattern {
        0 | 1 => {
            notes.sort_by(|a, b| match pattern {
                0 => b.note.partial_cmp(&a.note).unwrap(), // UP DN
                _ => a.note.partial_cmp(&b.note).unwrap(), // DN UP
            });
        },
        _ => {}
    }
    let mut i: f32 = 0.0;
    for mut note in notes.iter_mut() {
        note.t_in = (i * samples_per_event) as Offset;
        note.t_out = ((i + 1.0) * samples_per_event) as Offset;
        note.id = i as u16;
        i = i + 2.0;
    }
}

pub fn dispatch(store: &mut Store, action: Action) {
    match action {
        Action::NoteOn(note, vel) => {
            // Push a new note to the end of store.notes 
            // ... and redistribute the t_in and t_out 
            // ... based on the rate and samples per bar
            store.notes.push(Note {
                id: 0,
                r_id: 0,
                t_in: 0,
                t_out: 0,
                note, 
                vel,
            });
            distribute_notes(
                store.notes.borrow_mut(), 
                store.bar,
                store.pattern);
        },
        Action::NoteOff(note) => {
            store.notes.retain(|n| n.note != note);
            if store.notes.len() > 0 {
                distribute_notes(
                    store.notes.borrow_mut(), 
                    store.bar,
                    store.pattern);
            }
            store.queue.push(Action::NoteOff(note));
        },
        Action::SetParam(ref key, val) if key == "length" => {
            store.length = val;
            store.bar = calculate_beat(
                store.sample_rate, 
                store.bpm, 
                store.length);
            if store.notes.len() > 0 {
                distribute_notes(
                    store.notes.borrow_mut(), 
                    store.bar, 
                    store.pattern);
            }
        }
        Action::SetParam(ref key, val) if key == "pattern" => {
            store.pattern = val as usize;
            if store.notes.len() > 0 {
                distribute_notes(
                    store.notes.borrow_mut(), 
                    store.bar,
                    store.pattern);
            }
        }
        _ => {}
    }
}

pub fn read(doc: &mut Element) -> Option<Store> {
    let (mut doc, params) = param_map(doc);
    let mut store: Store = init();
    store.length = *params.get("length").unwrap_or(&4.0);
    store.bar = calculate_beat(store.sample_rate, store.bpm, store.length);
    Some(store)
}

pub fn compute(store: &mut Store) {
    for note in store.notes.iter() {
        if store.timer == note.t_in {
            store.queue.push(Action::NoteOn(note.note, note.vel));
        }
        if store.timer == note.t_out {
            store.queue.push(Action::NoteOff(note.note));
        }
    }
    // LOOP
    store.timer = if store.timer >= store.bar { 0 } 
        else { store.timer + 1 }
}

// The issue with requesting dispatch from nodes once
// ... every buffer loop is that the state of any timer
// ... stored within the module at the time of request
// ... might not be up-to-date. Because of this we need
// ... to push actions into a queue at time of compute
// ... and then clear / dispatch the queue at request time
pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Actions for outputs
        Option<Vec<Action>>, // Actions for inputs
        Option<Vec<Action>> // Actions for client
    ) {
    if store.queue.len() == 0 { (None, None, None) } else {
        let carry = store.queue.clone();
        store.queue.clear();
        (Some(carry), None, None) 
    }
}
