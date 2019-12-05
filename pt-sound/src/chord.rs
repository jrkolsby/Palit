use xmltree::Element;

use crate::core::{Note, Key, Offset};
use crate::action::Action;
use crate::document::{note_list};

pub struct Store {
    thru_queue: Vec<Action>,
    intervals: Vec<Key>
}

pub fn init() -> Store {
    Store {
        thru_queue: vec![],
        intervals: vec![0,4,7], // Major Chord 1,4,7
    }
}

pub fn read(mut doc: Element) -> Option<Store> {
    let (mut doc, notes) = note_list(doc);
    let mut store: Store = init();
    store.intervals = notes;
    Some(store)
}

pub fn dispatch(store: &mut Store, action: Action) {
    match action {
        Action::NoteOn(note, vol) => {
            for dnote in store.intervals.iter() {
                println!("Noteon {}", note);
                store.thru_queue.push(Action::NoteOn(note+dnote, vol));
            }
        },
        Action::NoteOff(note) => {
            for dnote in store.intervals.iter() {
                store.thru_queue.push(Action::NoteOff(note+dnote));
            }
        },
        Action::SetParam(_, ctrl, value) => {
        },
        _ => {}
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Actions for outputs
        Option<Vec<Action>>, // Actions for inputs
        Option<Vec<Action>> // Actions for client
    ) {
        let carry = store.thru_queue.clone();
        store.thru_queue.clear();
        (Some(carry), None, None)
}