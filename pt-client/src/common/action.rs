use crate::common::Anchor;

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

    Route,
    RouteAnchors(Vec<Anchor>),
    PatchAnchor(u16),
    PatchRoute(u16),
    AddRoute(u16),
    FadePatch(u16, f32),

    NoteOn(u16, f32),
    NoteOff(u16),
    SetParam(u16, f32),
    MoveRegion(u16, u16, u32), // module id, region id, offset
    Solo(u16),
    Record(u16),
    Mute(u16),

    // Default actions
    OpenProject(String),
    CreateProject(String),
    Pepper,
    InputTitle,
    Noop,

    Error(String),
}

