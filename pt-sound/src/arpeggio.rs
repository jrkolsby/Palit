use crate::core::{Note, Key, Offset};
use crate::action::Action;

pub struct Store {
    timer: Offset,
    length: Offset,
    notes: Vec<Note>,
    pattern: Vec<Key>,
    queue: Vec<Action>,
}

pub fn init() -> Store {
    Store {
        timer: 0,
        length: 24000,
        notes: vec![
            Note {
                t_in: 0,
                t_out: 5000,
                note: 69,
                vel: 0.4,
            },
            Note {
                t_in: 5000,
                t_out: 24000,
                note: 71,
                vel: 0.4,
            },
            Note {
                t_in: 8000,
                t_out: 24000,
                note: 75,
                vel: 0.4,
            },
            Note {
                t_in: 15000,
                t_out: 16000,
                note: 58,
                vel: 1.0,
            },
        ],
        // This is the chromatic order in which notes are
        // ... inserted into store.notes where 0 is the 
        pattern: vec![0, 2, 1, 3],
        queue: vec![],
    }
}

pub fn dispatch(store: &mut Store, action: Action) {
    match action {
        Action::NoteOn(note, vol) => {
            //store.notes.push(Note {})
        },
        Action::NoteOff(note) => {
            
        },
        Action::SetParam(_, ctrl, value) => {
        }
        _ => {}
    }
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