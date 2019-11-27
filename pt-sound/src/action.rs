use crate::core::{Offset, Volume, Key};

#[derive(Debug, Clone)]
pub enum Action {
    // true = on
    LoopMode(bool),

    // true = up
    Octave(bool), 
    Volume(bool), 

    AddRoute(u16),

    PatchIn(u16, u16, u16),
    PatchOut(u16, u16, u16),

    DeleteRoute(u16),

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),

    Pepper,
    InputTitle,

    NoteOn(Key, Volume),
    NoteOff(Key),

    SetParam(usize, u32, i32),

    Arm(Offset),

    MoveRegion(u16, u16, u16, u16), // Module ID, region ID, new track, new offset


    Play,
    Stop,

    Tick,

    Exit,

    Noop,
}

