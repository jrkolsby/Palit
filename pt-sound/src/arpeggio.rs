use std::borrow::BorrowMut;
use xmltree::Element;

use crate::document::{param_map};
use crate::core::{Note, Key, Offset};
use crate::action::Action;

pub struct Store {
    timer: Offset,
    length: Offset,
    notes: Vec<Note>,
    queue: Vec<Action>,
}

pub fn init() -> Store {
    Store {
        timer: 0,
        length: 24000, // samples per bar/rate

        notes: vec![],
        queue: vec![],
    }
}

fn distribute_notes(notes: &mut Vec<Note>, length: Offset) {
    let len_i = notes.len() as u32;
    // given 3 notes in arpeggio, evenly space 6 NoteOn / NoteOff
    // actions, starting with the first event at timer == 0
    // |O  F  O  F  O  F  |
    let samples_per_event = length / (len_i*2);
    let mut i = 0;
    for mut note in notes.iter_mut() {
        note.t_in = i*samples_per_event;
        note.t_out = (i+1)*samples_per_event;
        note.id = i as u16;
        i = i+2;
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
                t_in: 0,
                t_out: 0,
                note, 
                vel,
            });
            distribute_notes(store.notes.borrow_mut(), store.length);
        },
        Action::NoteOff(note) => {
            println!("arp off {}", note);
            store.notes.retain(|n| n.note != note);
            if store.notes.len() > 0 {
                distribute_notes(store.notes.borrow_mut(), store.length);
            }
            store.queue.push(Action::NoteOff(note));
        },
        _ => {}
    }
}

pub fn read(doc: &mut Element) -> Option<Store> {
    let (mut doc, params) = param_map(doc);
    let mut store: Store = init();
    store.length = match params.get("length") {
        Some(a) => (*a * 1000) as Offset,
        None => return None,
    };
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
    store.timer = if store.timer == store.length { 0 } 
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
