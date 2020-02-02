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

    Octave(bool), // true = up
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
    LoopMode(u16, bool), // true = on

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
    PatchOut(u16, usize, u16),
    PatchIn(u16, usize, u16),
    DelPatch(u16, usize, bool),

    NoteOn(Key, Volume),
    NoteOff(Key),

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
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let argv: Vec<&str> = raw.split(":").collect();
        Ok(match argv[0] {
            "EXIT" => Action::Exit,
            "PLAY" => Action::PlayAt(argv[1].parse().unwrap()),
            "STOP" => Action::StopAt(argv[1].parse().unwrap()),
            "RECORD_AT" => Action::RecordAt(argv[1].parse().unwrap(),
                                            argv[2].parse().unwrap(),
                                            argv[3].parse().unwrap()),
            "MUTE_AT" => Action::MuteAt(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap(),
                                        argv[3] == "1"),
            "MUTE_AT" => Action::SoloAt(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap(),
                                        argv[3] == "1"),
            "MUTE_AT" => Action::MonitorAt(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap(),
                                        argv[3] == "1"),
            "NOTE_ON" => Action::NoteOn(argv[1].parse().unwrap(), 
                                        argv[2].parse().unwrap()),
            "NOTE_OFF" => Action::NoteOff(argv[1].parse().unwrap()),
            "NOTE_ON_AT" => Action::NoteOnAt(argv[1].parse().unwrap(),
                                             argv[2].parse().unwrap(),
                                             argv[3].parse().unwrap()),
            "NOTE_OFF_AT" => Action::NoteOffAt(argv[1].parse().unwrap(),
                                               argv[2].parse().unwrap()),
            "OCTAVE" => Action::Octave(argv[1] == "1"),
            "SCRUB" => Action::Scrub(argv[1].parse().unwrap(),
                                     argv[2] == "1"),
            "OPEN_PROJECT" => Action::OpenProject(argv[1].to_string()),
            "PATCH_OUT" => Action::PatchOut(argv[1].parse().unwrap(),
                                            argv[2].parse().unwrap(),
                                            argv[3].parse().unwrap()),
            "PATCH_IN" => Action::PatchIn(argv[1].parse().unwrap(),
                                          argv[2].parse().unwrap(),
                                          argv[3].parse().unwrap()),
            "DEL_PATCH" => Action::DelPatch(argv[1].parse().unwrap(),
                                            argv[2].parse().unwrap(),
                                            argv[3] == "1"),
            "DEL_ROUTE" => Action::DelRoute(argv[1].parse().unwrap()),
            "ADD_ROUTE" => Action::AddRoute(argv[1].parse().unwrap()),
            "SET_PARAM" => Action::SetParam(argv[1].parse().unwrap(),
                                            argv[2].to_string(),
                                            argv[3].parse().unwrap()),
            "GOTO" => Action::GotoAt(argv[1].parse().unwrap(),
                                     argv[2].parse().unwrap()),
            "SET_TEMPO" => Action::SetTempo(argv[1].parse().unwrap()),
            "SET_METER" => Action::SetMeter(argv[1].parse().unwrap(),
                                            argv[2].parse().unwrap()),
            "LOOP_MODE" => Action::LoopMode(argv[1].parse().unwrap(),
                                            argv[2] == "1"),
            "SET_LOOP" => Action::SetLoop(argv[1].parse().unwrap(),
                                          argv[2].parse().unwrap(),
                                          argv[3].parse().unwrap()),
            "ADD_MODULE" => Action::AddModule(argv[1].parse().unwrap(),
                                              argv[2].to_string()),
            "DEL_MODULE" => Action::DelModule(argv[1].parse().unwrap()),
            _ => return Err(raw.to_string())
        })
    }
}