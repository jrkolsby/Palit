use crate::core::{Note, Key, Offset};
use crate::action::Action;

pub struct Store {
    timer: Offset,
    notes: Vec<Note>,
    pattern: Vec<Key>,
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