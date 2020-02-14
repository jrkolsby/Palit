use crate::pcm::{Offset, Volume, Key, Note, Anchor, Module, Param};
use std::str::FromStr;

/*
    PALIT IPC PROTOCOL
    Actions are formatted as strings with an optional module ID specifier
    to which the action should be dispatched. The format is as follows:
    "<n_id>@<action>:<arg1>:<arg2>:<arg3> "
    Actions are received via pipe and are delimited by a space. Multiple
    actions may be read by the event loop simultaneously 

    n_id Node ID
    r_id Route ID
    a_id Anchor ID (Any input or output from a module)
    op_id Module Operator ID (Dispatches to a cluster of nodes)
    m_id Module ID
    t_id => Track ID
*/

#[derive(Debug, Clone)]
pub enum Action {
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
    Route,
    Cancel,
    At(u16, Box<Action>), // A direct action
    NoteOn(Key, Volume),
    NoteOff(Key),
    Goto(Offset),
    Tick(Offset),
    Octave(bool), // true = up
    Volume(bool), 
    SetTempo(u16),
    AddNote(Note),
    Scrub(bool),
    SetLoop(Offset, Offset),
    LoopMode(bool), // true = on
    LoopOff,
    Loop(Offset, Offset),
    AddModule(u16, String),
    OpenProject(String),
    CreateProject(String),
    SetParam(String, Param),
    DeclareParam(String, f32, f32, f32, f32),
    DeclareAnchors(usize, usize),
    SoloTrack(u16, bool), // Track ID, is_on
    MuteTrack(u16, bool),
    MonitorTrack(u16, bool),
    RecordTrack(u16, u8), // Track ID, mode (0 off, 1 midi, 2 audio)
    SetMeter(u16, u16),
    MoveRegion(u16, u16, Offset), // region ID, new track, new offset
    // Track ID, Region ID, offset, duration, wav_dest
    AddRegion(u16, u16, u16, Offset, Offset, String),
    ShowAnchors(Vec<Anchor>), 
    PatchAnchor(u16),
    PatchRoute(u16),
    AddRoute(u16),
    DelRoute(u16),
    FadePatch(u16, f32),
    PatchOut(u16, u16, u16),
    PatchIn(u16, u16, u16),
    DelPatch(u16, u16, bool),
    TryoutModule(u16),
    DelModule(u16),
    ShowProject(String, Vec<Module>),
    InputTitle,
    Noop,
    Error(String),
    Exit,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        format!("{} ", match self {
            Action::At(n_id, action) => format!("{}@{}", n_id, action.to_string()),
            Action::NoteOn(key, vel) => format!("NOTE_ON:{}:{}", key, vel),
            Action::NoteOff(key) => format!("NOTE_OFF:{}", key),
            Action::AddNote(n) => format!("NOTE_ADD:{}:{}:{}:{}:{}",
                n.id, n.note, n.vel, n.t_in, n.t_out),
            Action::AddRegion(t_id, r_id, a_id, offset, duration, source) => 
                format!("REGION_ADD:{}:{}:{}:{}:{}:{}",
                    t_id, r_id, a_id, offset, duration, source),
            Action::Scrub(dir) => format!("SCRUB:{}",
                if *dir { "1" } else { "0" }),
            Action::MuteTrack(t_id, is_on) => format!("MUTE_TRACK:{}:{}", 
                t_id, if *is_on { "1" } else { "0" }),
            Action::RecordTrack(t_id, is_on) => format!("RECORD_TRACK:{}:{}", 
                t_id, match *is_on {
                    1 => "1",
                    2 => "2",
                    _ => "0",
                }
            ),
            Action::SoloTrack(t_id, is_on) => 
                format!("SOLO_TRACK:{}:{}", t_id, if *is_on { "1" } else { "0" }),
            Action::MonitorTrack(t_id, is_on) => 
                format!("MONITOR_TRACK:{}:{}", t_id, if *is_on { "1" } else { "0" }),
            Action::PatchOut(module_id, anchor_id, route_id) => 
                format!("PATCH_OUT:{}:{}:{}", module_id, anchor_id, route_id),
            Action::PatchIn(module_id, anchor_id, route_id) => 
                format!("PATCH_IN:{}:{}:{}", module_id, anchor_id, route_id),
            Action::DelPatch(module_id, anchor_id, is_input) => 
                format!("DEL_PATCH:{}:{}:{}", module_id, anchor_id, 
                    if *is_input {"1"} else {"2"}),
            Action::DelRoute(route_id) => format!("DEL_ROUTE:{}", route_id),
            Action::AddRoute(route_id) => format!("ADD_ROUTE:{}", route_id),
            Action::SetParam(key, val) => format!("SET_PARAM:{}:{}", key, val),
            Action::DeclareParam(key, init, min, max, step) => 
                format!("DECLARE_PARAM:{}:{}:{}:{}:{}", key, init, min, max, step),
            Action::DeclareAnchors(ins, outs) => 
                format!("DECLARE_ANCHORS:{}:{}", ins, outs),
            Action::SetMeter(beat, note) => format!("SET_METER:{}:{}", beat, note),
            Action::SetTempo(tempo) => format!("SET_TEMPO:{}", tempo),
            Action::SetLoop(l_in, l_out) => format!("SET_LOOP:{}:{}", l_in, l_out),
            Action::LoopMode(is_on) => format!("LOOP_MODE:{}", 
                if *is_on { "1" } else { "0" }
            ),
            Action::Goto(playhead) => format!("GOTO:{}", playhead),
            Action::Tick(offset) => format!("TICK:{}", offset),
            Action::Play => format!("PLAY"),
            Action::Stop => format!("STOP"),
            _ => "NOOP".to_string()
        })
    }
}

impl FromStr for Action {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let argv: Vec<&str> = raw.split(":").collect();
        let argv0: Vec<&str> = argv[0].split("@").collect();
        let is_direct = argv0.len() > 1;
        let title = if is_direct { argv0[1] } else { argv0[0] };
        let action = match title {
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
            "RECORD" => Action::Record,
            "DESELECT" => Action::Deselect,
            "EFFECT" | "INSTRUMENT" => Action::Instrument,
            "RECORD_TRACK" => Action::RecordTrack(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap()),
            "MUTE_TRACK" => Action::MuteTrack(
                argv[1].parse().unwrap(),
                argv[2] == "1"),
            "SOLO_TRACK" => Action::SoloTrack(
                argv[1].parse().unwrap(),
                argv[2] == "1"),
            "MONITOR_TRACK" => Action::MonitorTrack(
                argv[1].parse().unwrap(),
                argv[2] == "1"),
            "NOTE_ON" => Action::NoteOn(
                argv[1].parse().unwrap(), 
                argv[2].parse().unwrap()),
            "NOTE_OFF" => Action::NoteOff(argv[1].parse().unwrap()),
            "OCTAVE" => Action::Octave(argv[1] == "1"),
            "SCRUB" => Action::Scrub(argv[1] == "1"),
            "OPEN_PROJECT" => Action::OpenProject(argv[1].to_string()),
            "PATCH_OUT" => Action::PatchOut(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap(),
                argv[3].parse().unwrap()),
            "PATCH_IN" => Action::PatchIn(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap(),
                argv[3].parse().unwrap()),
            "DEL_PATCH" => Action::DelPatch(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap(),
                argv[3] == "1"),
            "DEL_ROUTE" => Action::DelRoute(argv[1].parse().unwrap()),
            "ADD_ROUTE" => Action::AddRoute(argv[1].parse().unwrap()),
            "SET_PARAM" => Action::SetParam(
                argv[1].to_string(),
                argv[2].parse().unwrap()),
            "DECLARE_PARAM" => Action::DeclareParam(
                argv[1].to_string(),
                argv[2].parse().unwrap(),
                argv[3].parse().unwrap(),
                argv[4].parse().unwrap(),
                argv[5].parse().unwrap()),
            "DECLARE_ANCHORS" => Action::DeclareAnchors(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap()),
            "GOTO" => Action::Goto(argv[2].parse().unwrap()),
            "SET_TEMPO" => Action::SetTempo(argv[1].parse().unwrap()),
            "SET_METER" => Action::SetMeter(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap()),
            "LOOP_MODE" => Action::LoopMode(argv[1] == "1"),
            "SET_LOOP" => Action::SetLoop(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap()),
            "ADD_MODULE" => Action::AddModule(
                argv[1].parse().unwrap(),
                argv[2].to_string()),
            "DEL_MODULE" => Action::DelModule(argv[1].parse().unwrap()),
            "TICK" => Action::Tick(argv[1].parse().unwrap()),
            "NOTE_ADD" => Action::AddNote(Note {
                id: argv[1].parse().unwrap(),
                note: argv[2].parse().unwrap(),
                vel: argv[3].parse().unwrap(),
                t_in: argv[4].parse().unwrap(),
                t_out: argv[5].parse().unwrap(),
            }),
            "REGION_ADD" => Action::AddRegion(
                argv[1].parse().unwrap(),
                argv[2].parse().unwrap(),
                argv[3].parse().unwrap(),
                argv[4].parse().unwrap(),
                argv[5].parse().unwrap(),
                argv[6].to_string()),
            _ => return Err(raw.to_string())
        };
        Ok(if is_direct { Action::At(
            argv0[0].parse().unwrap(), 
            Box::new(action)) 
        } else { 
            action 
        })
    }
}