#[derive(Debug, Clone)]
pub enum Action {

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

    // MAIN DISPATCH 
    SwapNode,
    RemoveRoute,
    AddRoute,

    Noop,
}

