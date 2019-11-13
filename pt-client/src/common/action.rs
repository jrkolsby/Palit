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
    Redo,
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
    Deselect,
    Record,

    NoteOn(u16, f32),
    NoteOff(u16),
    SetParam(u16, f32),
    PatchIn(u16, u16), // route id, input id
    PatchOut(u16, u16), // route id, output id
    MoveRegion(u16, u16, u32), // module id, region id, offset

    // Default actions
    OpenProject(String),
    CreateProject(String),
    Pepper,
    InputTitle,
    Noop,
}

