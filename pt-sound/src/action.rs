use dsp::{NodeIndex, EdgeIndex};

#[derive(Debug, Clone)]
pub enum Action {
    // KEYBOARD ACTIONS
    LoopMode(bool),

    PitchUp,
    PitchDown,

    VolumeUp,
    VolumeDown,

    AddRoute(NodeIndex, u8, NodeIndex, u8),
    DeleteRoute(EdgeIndex),

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),

    Pepper,
    InputTitle,

    NoteOn(u8, f64),
    NoteOff(u8),

    SetParam(usize, u32, i32),

    Play,
    Stop,

    Tick,

    Exit,

    Noop,
}

