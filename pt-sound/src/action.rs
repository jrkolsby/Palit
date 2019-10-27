use dsp::{NodeIndex, EdgeIndex};
use crate::core::{Offset, Volume, Key};

#[derive(Debug, Clone)]
pub enum Action {
    // true = on
    LoopMode(bool),

    // true = up
    Octave(bool), 
    Volume(bool), 

    AddRoute(NodeIndex, u8, NodeIndex, u8),
    DeleteRoute(EdgeIndex),

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),

    Pepper,
    InputTitle,

    NoteOn(Key, Volume),
    NoteOff(Key),

    SetParam(usize, u32, i32),

    Arm(Offset),

    Play,
    Stop,

    Tick,

    Exit,

    Noop,
}

