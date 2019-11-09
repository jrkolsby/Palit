#[derive(Debug, Clone)]
pub enum Action {
    // KEYBOARD ACTIONS
    Up,
    Down,
    Left,
    Right,
    SelectR,
    SelectB,
    SelectG,
    SelectY,
    SelectP,
    MidiEvent,
    RotaryEvent,
    Effect,
    Route,
    Instrument,
    Undo,
    Do,
    Shift,
    Back,
    Save,
    Play,
    Stop,
    In,
    Out,
    Edit,
    LoopMode(bool),
    PitchUp,
    PitchDown,
    VolumeUp,
    VolumeDown,
    Help,
    Tick,
    Exit,

    NoteOn(u16, f32),
    NoteOff(u16),
    SetParam(u16, f32),

    // ABSTRACT ACTIONS
    OpenProject(String),
    CreateProject(String),
    Pepper,
    InputTitle,

    Deselect,

    Noop,
}

pub type DirectAction = (u16, Action);
