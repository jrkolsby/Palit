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

    Play,
    Stop,

    Noop,
}

