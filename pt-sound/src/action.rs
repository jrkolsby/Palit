use crate::core::{Offset, Volume, Key, Note};

#[derive(Debug, Clone)]
pub enum Action {
    // true = on
    LoopOff(u16),
    Loop(u16, Offset, Offset),

    // true = up
    Octave(bool), 
    Volume(bool), 

    AddModule(u16, String),
    DelModule(u16),

    // Node ID, I/O ID, Route ID
    PatchIn(u16, usize, u16),
    PatchOut(u16, usize, u16),
    DelRoute(u16),
    DelPatch(u16, usize, bool),
    AddRoute(u16),

    // Default actions
    OpenProject(String),
    CreateProject(String),
    Pepper,
    InputTitle,
    Tick(Offset),

    NoteOn(Key, Volume),
    NoteOff(Key),

    NoteOnAt(u16, Key, Volume),
    NoteOffAt(u16, Key),
    SetParam(u16, String, i32),

    // Direct actions
    Goto(u16, Offset),
    Play(u16),
    Stop(u16),
    Record(u16),
    Monitor(u16),
    RecordAt(u16, u16),
    MuteAt(u16, u16),
    AddNote(u16, Note),
    Scrub(u16, bool),
    SetLoop(u16, Offset, Offset),
    LoopMode(u16, bool),

    // Global actions
    SetTempo(u16),
    SetMeter(u16, u16),

    // Module ID, region ID, new track, new offset
    MoveRegion(u16, u16, u16, u16), 

    Exit,
    Noop,
}

