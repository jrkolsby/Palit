use dsp::NodeIndex;

#[derive(Debug, Clone)]
pub enum Action {
    // KEYBOARD ACTIONS
    LoopMode(bool),

    PitchUp,
    PitchDown,
    VolumeUp,
    VolumeDown,

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),

    Pepper,
    InputTitle,

    NoteOn(u8, f64),
    NoteOff(u8),

    SetParam(NodeIndex, u8, f64),

    Play,
    Stop,

    Noop,
}

