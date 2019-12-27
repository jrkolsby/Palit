use crate::core::{Offset, Volume, Key};

#[derive(Debug, Clone)]
pub enum Action {
    // true = on
    LoopMode(bool),

    // true = up
    Octave(bool), 
    Volume(bool), 

    AddModule(u16, String),

    // Node ID, I/O ID, Route ID
    PatchIn(u16, usize, u16),
    PatchOut(u16, usize, u16),
    DeleteRoute(u16),
    DeletePatch(u16, usize, bool),
    AddRoute(u16),

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),

    Pepper,
    InputTitle,

    NoteOn(Key, Volume),
    NoteOff(Key),

    NoteOnAt(u16, Key, Volume),
    NoteOffAt(u16, Key),

    SetParam(u16, String, i32),

    Arm(Offset),

    // Module ID, region ID, new track, new offset
    MoveRegion(u16, u16, u16, u16), 

    Play(u16),
    Stop(u16),

    Tick,

    Exit,

    Noop,
}

