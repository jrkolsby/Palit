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
    Record,
    Goto(u32),
    Scrub(bool),

    // Patching actions
    Route,
    ShowAnchors(Vec<Anchor>), 
    PatchAnchor(u16),
    PatchRoute(u16),
    AddRoute(u16),
    DelRoute(u16),
    FadePatch(u16, f32),
    PatchOut(u16, u16, u16),
    PatchIn(u16, u16, u16),
    DelPatch(u16, u16, bool),

    NoteOn(u16, f32),
    NoteOff(u16),
    SetParam(String, i16),
    MoveRegion(u16, u16, u32), // module id, region id, offset
    SoloTrack(u16),
    RecordTrack(u16),
    MuteTrack(u16),
    PlayAt(u16),
    AddNote(u16, u8, f32, u32, u32),

    // Default actions
    OpenProject(String),
    CreateProject(String),
    Pepper,
    InputTitle,
    Noop,

    Error(String),
}

