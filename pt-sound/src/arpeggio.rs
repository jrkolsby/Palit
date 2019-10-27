use crate::core::{Note, Key, Offset};
use crate::action::Action;

pub struct Store {
    timer: Offset,
    notes: Vec<Note>,
    pattern: Vec<Key>,
}

pub fn init() -> Store {
    Store {
        timer: 0,
        notes: vec![
            Note {
                t_in: 0,
                t_out: 48000,
                note: 69,
                vel: 0.4,

            },
        ],
        pattern: vec![0, 2, 1, 3]
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
    store.timer += 1;
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Actions for outputs
        Option<Vec<Action>>, // Actions for inputs
        Option<Vec<Action>> // Actions for client
    ) {
    let mut out_a = Vec::new();
    for note in store.notes.iter() {
        if store.timer == note.t_in {
            out_a.push(Action::NoteOn(note.note, note.vel));
        }
        if store.timer == note.t_out {
            out_a.push(Action::NoteOff(note.note));
        }
    }
    if out_a.len() > 0 {
        (Some(out_a), None, None)
    } else {
        (None, None, None)
    }
}