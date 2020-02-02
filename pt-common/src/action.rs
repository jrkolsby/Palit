use crate::pcm::{Offset, Volume, Key, Note, Anchor, Module};
use std::str::FromStr;

/*
    n_id Node ID
    r_id Route ID
    a_id Anchor ID (Any input or output from a module)
    op_id Module Operator ID (Dispatches to a cluster of nodes)
    m_id Module ID
    t_id => Track ID
*/

pub struct ActionError;

//#[derive(Debug, Clone)]
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
    PitchUp,
    PitchDown,
    VolumeUp,
    VolumeDown,
    Help,
    Deselect,
    Record,

    Goto(u32),
    SetTempo(u16),

    // true = on
    LoopOff(u16),
    Loop(u16, Offset, Offset),

    // true = up
    Octave(bool), 
    Volume(bool), 

    AddModule(u16, String),

    // Default actions
    OpenProject(String),
    CreateProject(String),
    Tick(Offset),

    NoteOnAt(u16, Key, Volume),
    NoteOffAt(u16, Key),
    SetParam(u16, String, i32),

    // Direct actions
    GotoAt(u16, Offset),
    PlayAt(u16),
    StopAt(u16),

    MonitorAt(u16, u16, bool),
    RecordAt(u16, u16, u8),
    MuteAt(u16, u16, bool),
    SoloAt(u16, u16, bool),

    AddNote(u16, Note),
    Scrub(u16, bool),
    SetLoop(u16, Offset, Offset),
    LoopMode(u16, bool),

    // Global actions
    SetMeter(u16, u16),

    // Module ID, region ID, new track, new offset
    MoveRegion(u16, u16, u16, u16), 
    // Module ID, Track ID, Region ID, offset, duration, wav_dest
    AddRegion(u16, u16, u16, u16, Offset, Offset, String),

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

    SoloTrack(u16, bool), // Track ID, is_on
    MuteTrack(u16, bool),
    MonitorTrack(u16, bool),
    RecordTrack(u16, u8), // Track ID, mode (0 off, 1 midi, 2 audio)


    TryoutModule(u16),
    DelModule(u16),
    ShowProject(String, Vec<Module>),
    InputTitle,
    Cancel,

    Error(String),
    Noop,
    Exit,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        "NOOP".to_string()
    }
}

impl FromStr for Action {
    type Err = ActionError;
    fn from_str(s: &str) -> Result<Action, ActionError> {
        Ok(Action::Noop)
    }
}