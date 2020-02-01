use crate::common::{Anchor, Module};

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
    SetLoop(u32, u32),
    SetMeter(usize, usize),
    SetTempo(u16),
    PitchUp,
    PitchDown,
    VolumeUp,
    VolumeDown,
    Help,
    Exit,
    Deselect,
    Record,
    Goto(u32),
    Tick(u32),
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
    PlayAt(u16),

    // Track ID, is_on
    SoloTrack(u16, bool), 
    MuteTrack(u16, bool),
    MonitorTrack(u16, bool),
    // Track ID, mode (0 off, 1 midi, 2 audio)
    RecordTrack(u16, u8),

    // Note ID, Key, Velocity, t_on, t_off
    AddNote(u16, u8, f32, u32, u32), 
    // Module ID, Track ID, Region ID, Asset ID, offset, duration, src
    AddRegion(u16, u16, u16, u16, u32, u32, String),

    AddModule(String),
    TryoutModule(u16),
    DelModule(u16),
    OpenProject(String),
    CreateProject(String),
    ShowProject(String, Vec<Module>),
    Pepper,
    InputTitle,
    Cancel,
    Noop,

    Error(String),
}

