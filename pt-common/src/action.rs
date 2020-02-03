use crate::pcm::{Offset, Volume, Key, Note, Anchor, Module, Param};
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
    SetParam(u16, String, Param),

    // Direct actions
    GotoAt(u16, Offset),
    PlayAt(u16),
    StopAt(u16),

    SoloTrack(u16, bool), // Track ID, is_on
    SoloAt(u16, u16, bool),
    MuteTrack(u16, bool),
    MuteAt(u16, u16, bool),
    MonitorTrack(u16, bool),
    MonitorAt(u16, u16, bool),
    RecordTrack(u16, u8), // Track ID, mode (0 off, 1 midi, 2 audio)
    RecordAt(u16, u16, u8),

    AddNote(u16, Note),
    Scrub(u16, bool),
    SetLoop(u16, Offset, Offset),
    LoopMode(u16, bool), // true = on

    // Global actions
    SetMeter(u16, u16),

    // Module ID, region ID, new track, new offset
    MoveRegion(u16, u16, u16, Offset), 
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

    NoteOn(Key, Volume),
    NoteOff(Key),

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
        format!("{} ", match self {
            Action::Tick(offset) => format!("TICK:{}", offset),
            Action::NoteOn(key, vel) => format!("NOTE_ON:{}:{}", key, vel),
            Action::NoteOff(key) => format!("NOTE_OFF:{}", key),
            Action::AddNote(id, n) => format!("NOTE_ADD:{}:{}:{}:{}:{}",
                id, n.note, n.vel, n.t_in, n.t_out
            ),
            Action::AddRegion(n_id, t_id, r_id, a_id, offset, duration, source) => 
                format!("REGION_ADD:{}:{}:{}:{}:{}:{}:{}",
                    n_id, t_id, r_id, a_id, offset, duration, source
                ),
            Action::PlayAt(n_id) => format!("PLAY_AT:{}", n_id),
            Action::StopAt(n_id) => format!("STOP_AT:{}", n_id),
            Action::Scrub(n_id, dir) => format!("SCRUB:{}:{}", n_id, 
                if *dir { "1" } else { "0" }
            ),
            Action::MuteAt(n_id, t_id, is_on) => format!("MUTE_AT:{}:{}:{}", 
                n_id, t_id, if *is_on { "1" } else { "0" }
            ),
            Action::RecordAt(n_id, t_id, is_on) => format!("RECORD_AT:{}:{}:{}", 
                n_id, t_id, match *is_on {
                    1 => "1",
                    2 => "2",
                    _ => "0",
                }
            ),
            Action::SoloAt(n_id, t_id, is_on) => format!("SOLO_AT:{}:{}:{}", 
                n_id, t_id, if *is_on { "1" } else { "0" }
            ),
            Action::MonitorAt(n_id, t_id, is_on) => format!("MONITOR_AT:{}:{}:{}", 
                n_id, t_id, if *is_on { "1" } else { "0" }
            ),
            Action::NoteOnAt(n_id, key, vel) => format!("NOTE_ON_AT:{}:{}:{}", n_id, key, vel),
            Action::NoteOffAt(n_id, key) => format!("NOTE_OFF_AT:{}:{}", n_id, key),
            Action::PatchOut(module_id, anchor_id, route_id) => format!("PATCH_OUT:{}:{}:{}", 
                module_id, anchor_id, route_id
            ),
            Action::PatchIn(module_id, anchor_id, route_id) => format!("PATCH_IN:{}:{}:{}", 
                module_id, anchor_id, route_id
            ),
            Action::DelPatch(module_id, anchor_id, is_input) => format!("DEL_PATCH:{}:{}:{}", 
                module_id, anchor_id, if *is_input {"1"} else {"2"}
            ),
            Action::DelRoute(route_id) => format!("DEL_ROUTE:{}", route_id),
            Action::AddRoute(route_id) => format!("ADD_ROUTE:{}", route_id),
            Action::SetParam(n_id, key, val) => format!("SET_PARAM:{}:{}:{}", n_id, key, val),
            Action::SetMeter(beat, note) => format!("SET_METER:{}:{}", beat, note),
            Action::SetTempo(tempo) => format!("SET_TEMPO:{}", tempo),
            Action::SetLoop(n_id, l_in, l_out) => format!("SET_LOOP:{}:{}:{}", 
                n_id, l_in, l_out
            ),
            Action::LoopMode(n_id, is_on) => format!("LOOP_MODE:{}:{}", 
                n_id, if *is_on { "1" } else { "0" }
            ),
            Action::GotoAt(n_id, playhead) => format!("GOTO:{}:{}", n_id, playhead),
            _ => "NOOP".to_string()
        })
    }
}

impl FromStr for Action {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let argv: Vec<&str> = raw.split(":").collect();
        Ok(match argv[0] {
            "EXIT" => Action::Exit,
            "ROUTE" => Action::Route,
            "?" => Action::Noop,
            "1" => Action::Help,
            "2" => Action::Back,
            "PLAY" => Action::Play,
            "STOP" => Action::Stop,
            "M" => Action::SelectG,
            "R" => Action::SelectY,
            "V" => Action::SelectP,
            "I" => Action::SelectB,
            "SPC" => Action::SelectR,
            "UP" => Action::Up,
            "DN" => Action::Down,
            "LT" => Action::Left,
            "RT" => Action::Right,
            "EXIT" => Action::Exit,
            "DESELECT" => Action::Deselect,
            "EFFECT" | "INSTRUMENT" => Action::Instrument,
            "PLAY_AT" => Action::PlayAt(argv[1].parse().unwrap()),
            "STOP_AT" => Action::StopAt(argv[1].parse().unwrap()),
            "RECORD_AT" => Action::RecordAt(argv[1].parse().unwrap(),
                                            argv[2].parse().unwrap(),
                                            argv[3].parse().unwrap()),
            "MUTE_AT" => Action::MuteAt(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap(),
                                        argv[3] == "1"),
            "SOLO_AT" => Action::SoloAt(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap(),
                                        argv[3] == "1"),
            "MONITOR_AT" => Action::MonitorAt(argv[1].parse().unwrap(),
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
            "TICK" => Action::Tick(argv[1].parse().unwrap()),
            "OCTAVE" => if argv[1] == "1" { Action::PitchUp } else { Action::PitchDown },
            "NOTE_ON" => Action::NoteOn(argv[1].parse().unwrap(),
                                        argv[2].parse().unwrap()),
            "NOTE_OFF" => Action::NoteOff(argv[1].parse().unwrap()),
            "NOTE_ADD" => Action::AddNote(argv[1].clone().parse().unwrap(), Note {
                id: argv[1].parse().unwrap(),
                note: argv[2].parse().unwrap(),
                vel: argv[3].parse().unwrap(),
                t_in: argv[4].parse().unwrap(),
                t_out: argv[5].parse().unwrap(),
            }),
            "REGION_ADD" => Action::AddRegion(argv[1].parse().unwrap(),
                                      argv[2].parse().unwrap(),
                                      argv[3].parse().unwrap(),
                                      argv[4].parse().unwrap(),
                                      argv[5].parse().unwrap(),
                                      argv[6].parse().unwrap(),
                                      argv[7].to_string()),
            _ => return Err(raw.to_string())
        })
    }
}