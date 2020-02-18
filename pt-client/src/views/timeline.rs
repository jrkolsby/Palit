use std::io::{Write, Stdout};
use std::collections::HashMap;

use xmltree::Element;
use termion::cursor;
use libcommon::{Action, Anchor, Note, Param};

use crate::components::{tempo, button, ruler, region, roll};
use crate::common::{ID, VOID_ID, FocusType};
use crate::common::{MultiFocus, render_focii, shift_focus, generate_partial_waveform};
use crate::common::{Screen, Asset, Region, Track, Window};
use crate::common::{char_offset, offset_char, generate_waveforms};
use crate::modules::timeline;
use crate::views::{Layer};

use crate::common::{REGIONS_X, TIMELINE_Y};

static TRACKS_X: u16 = 3;
static ASSET_PREFIX: &str = "storage/";

#[derive(Clone, Debug)]
pub struct TimelineState {
    // Requires XML write/read
    pub tempo: u16,
    pub zoom: usize,
    pub temp_tempo: Option<u16>,
    pub temp_zoom: Option<usize>,
    pub meter_beat: u16,
    pub meter_note: u16,
    pub loop_mode: bool,
    pub seq_in: u32,
    pub seq_out: u32,
    pub loop_in: u32,
    pub loop_out: u32,
    pub sample_rate: u32,
    pub tracks: HashMap<u16, Track>,
    pub assets: HashMap<u16, Asset>,
    pub regions: HashMap<u16, Region>,
    pub notes: Vec<Note>,

    // Ephemeral variables
    pub tick: bool,
    pub playhead: u32,
    pub scroll_mid: u16,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub focus: (usize, usize),
}

// STATIC PROPERTIES THROUGHOUT VIEW'S LIFETIME
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    pub state: TimelineState,
    focii: Vec<Vec<MultiFocus<TimelineState>>>,
}

static VOID_RENDER: fn( &mut Screen, Window, ID, &TimelineState, bool) =
    |_, _, _, _, _| {};
static VOID_TRANSFORM: fn(Action, ID, &TimelineState) -> Action = 
    |_, _, _| Action::Noop;

fn generate_focii(tracks: &HashMap<u16, Track>, 
                  regions: &HashMap<u16, Region>) -> Vec<Vec<MultiFocus<TimelineState>>> {
    let mut focii: Vec<Vec<MultiFocus<TimelineState>>> = vec![vec![
        MultiFocus::<TimelineState> {
            r_id: (FocusType::Button, 0),
            r: |out, window, id, state, focus| 
                write!(out, "{} {} ", cursor::Goto(window.x + 2, window.y + 2), if state.loop_mode { 
                    "LOOP ON" 
                } else { 
                    "LOOP OFF"
                }).unwrap(),
            r_t: |a, id, state| match a { 
                Action::Up |
                Action::Down |
                Action::SelectR => Action::LoopMode(!state.loop_mode),
                _ => Action::Noop 
            },

            w: VOID_RENDER,
            w_id: VOID_ID.clone(),
            g_id: (FocusType::Button, 0),
            g: |out, window, id, state, focus| {
                let offset_out = char_offset(
                    state.loop_out,
                    state.sample_rate,
                    state.tempo,
                    state.zoom);
                let out_x = offset_out as i16 - state.scroll_x as i16;
                if out_x >= 0 {
                    write!(out, "{}>> ", cursor::Goto(
                        window.x + REGIONS_X + out_x as u16, 
                        window.y + TIMELINE_Y)
                    ).unwrap()
                }
            },
            g_t: |a, id, state| {
                let o1 = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                match a { 
                    Action::Left => Action::SetLoop(
                        state.loop_in, 
                        if state.loop_out >= o1 { state.loop_out - o1 }
                        else { state.loop_out }
                    ), 
                    Action::Right => Action::SetLoop(
                        state.loop_in, 
                        state.loop_out + o1
                    ), 
                    _ => Action::Noop 
                }
            },

            p_id: (FocusType::Button, 1),
            p: |out, window, id, state, focus| {
                let offset_in = char_offset(
                    state.loop_in,
                    state.sample_rate,
                    state.tempo,
                    state.zoom);
                let in_x = offset_in as i16 - state.scroll_x as i16;
                if in_x >= 0 {
                    write!(out, "{} <<", cursor::Goto(
                        window.x + REGIONS_X + in_x as u16 - 3, 
                        window.y + TIMELINE_Y)
                    ).unwrap()
                }
            },
            p_t: |a, id, state| {
                let o1 = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                match a { 
                    Action::Left => Action::SetLoop(
                        if state.loop_in >= o1 { state.loop_in - o1 } 
                        else { state.loop_in }, 
                        state.loop_out
                    ), 
                    Action::Right => Action::SetLoop(
                        state.loop_in + o1, 
                        state.loop_out
                    ), 
                    _ => Action::Noop 
                }
            },

            y_id: VOID_ID.clone(),
            y: VOID_RENDER,
            y_t: VOID_TRANSFORM,

            b_id: VOID_ID.clone(),
            b: VOID_RENDER,
            b_t: VOID_TRANSFORM,

            active: None,
        },
        MultiFocus::<TimelineState> {
            p_id: VOID_ID.clone(),
            p: VOID_RENDER,
            p_t: VOID_TRANSFORM,

            g_id: (FocusType::Param, 0),
            g: |out, window, id, state, focus| {
                let zoom = if let Some(z) = state.temp_zoom { z } else { state.zoom };
                write!(out, "{} {}X ", cursor::Goto(
                    window.x+window.w - 19, 2
                ), zoom).unwrap();
            },
            g_t: |a, id, state| {
                let zoom = if let Some(z) = state.temp_zoom { z } else { state.zoom };
                match a {
                    Action::Up => Action::Zoom(zoom + 1),
                    Action::Down => {
                        if zoom > 0 {
                            Action::Zoom(zoom - 1)
                        } else {
                            Action::Noop
                        }
                    },
                    _ => Action::Noop,
                }
            },

            r_id: (FocusType::Param, 0),
            r: |out, window, id, state, focus| {
                let tempo = if let Some(t) = state.temp_tempo { t } else { state.tempo };
                tempo::render(out, window.x+window.w-3, window.y, tempo, state.tick);
            },
            r_t: |a, id, state| {
                let tempo = if let Some(t) = state.temp_tempo { t } else { state.tempo };
                match a {
                    Action::Up => Action::SetTempo(tempo + 1),
                    Action::Down => if tempo > 0 {
                        Action::SetTempo(tempo - 1)
                    } else {
                        Action::Noop
                    },
                    _ => Action::Noop
                }
            },

            y_id: (FocusType::Param, 0),
            y: |out, window, id, state, focus|
                write!(out, "{} {} ", cursor::Goto(
                    window.x+window.w-14, 2
                ), state.meter_beat).unwrap(),
            y_t: |a, id, state| match a {
                Action::Up => Action::SetMeter(state.meter_beat + 1, state.meter_note),
                Action::Down => Action::SetMeter(state.meter_beat - 1, state.meter_note),
                _ => Action::Noop,
            },
            
            b_id: (FocusType::Param, 0),
            b: |out, window, id, state, focus|
                write!(out, "{} {} ", cursor::Goto(
                    window.x+window.w-14, 3
                ), state.meter_note).unwrap(),
            b_t: |a, id, state| match a {
                Action::Up => Action::SetMeter(state.meter_beat, state.meter_note + 1),
                Action::Down => Action::SetMeter(state.meter_beat, state.meter_note - 1),
                _ => Action::Noop,
            },

            w: VOID_RENDER,
            w_id: VOID_ID.clone(),

            active: None,
        },
    ]];

    let mut track_vec: Vec<(&u16, &Track)> = tracks.iter().collect();
    track_vec.sort_by(|(_, a), (_, b)| a.index.cmp(&b.index));

    for (t_id, track) in track_vec.iter() {
        focii.push(vec![
            MultiFocus::<TimelineState> {
                w: VOID_RENDER,
                w_id: (FocusType::Button, **t_id),

                r_id: VOID_ID.clone(),
                g_id: VOID_ID.clone(),
                y_id: VOID_ID.clone(),
                p_id: VOID_ID.clone(),
                b_id: VOID_ID.clone(),

                r: |mut out, win, id, state, focus| {
                    let x = win.x + TRACKS_X;
                    let y = win.y + TIMELINE_Y + 2 * id.1;
                    write!(out, "{}{}", cursor::Goto(x, y),
                        match state.tracks.get(&id.1).unwrap().record {
                            1 => "…",
                            2 => "∿",
                            _ => "",
                        }
                    ).unwrap();
                    write!(out, "{}{}", cursor::Goto(x, y + 1),
                        match state.tracks.get(&id.1).unwrap().record {
                            0 => "r",
                            _ => "R"
                        }
                    ).unwrap();
                },
                r_t: |action, id, state| match action {
                    Action::SelectR => Action::RecordTrack(
                        id.1, 
                        (state.tracks.get(&id.1).unwrap().record + 1) % 3
                    ),
                    _ => Action::Noop,
                },

                g: |mut out, win, id, state, focus| {
                    let x = win.x + TRACKS_X + 2;
                    let y = win.y + TIMELINE_Y + 2 * id.1;
                    write!(out, "{}{}", cursor::Goto(x, y),
                        if state.tracks.get(&id.1).unwrap().mute { "." } else { "" }
                    ).unwrap();
                    write!(out, "{}{}", cursor::Goto(x, y + 1),
                        if state.tracks.get(&id.1).unwrap().mute { "M" } else { "m" }
                    ).unwrap();
                },
                g_t: |action, id, state| match action {
                    Action::SelectG => Action::MuteTrack(
                        id.1,
                        !state.tracks.get(&id.1).unwrap().mute
                    ),
                    _ => Action::Noop,
                },

                b: |mut out, win, id, state, focus| {
                    let x = win.x + TRACKS_X + 4;
                    let y = win.y + TIMELINE_Y + 2 * id.1;
                    write!(out, "{}{}", cursor::Goto(x, y),
                        if state.tracks.get(&id.1).unwrap().solo { "„" } else { "" }
                    ).unwrap();
                    write!(out, "{}{}", cursor::Goto(x, y + 1),
                        if state.tracks.get(&id.1).unwrap().solo { "S" } else { "s" }
                    ).unwrap();
                },
                b_t: |action, id, state| match action {
                    Action::SelectB => Action::SoloTrack(
                        id.1,
                        !state.tracks.get(&id.1).unwrap().solo
                    ),
                    _ => Action::Noop,
                },

                p: |mut out, win, id, state, focus| {
                    let x = win.x + TRACKS_X + 6;
                    let y = win.y + TIMELINE_Y + 2 * id.1;
                    write!(out, "{}{}", cursor::Goto(x, y),
                        if state.tracks.get(&id.1).unwrap().monitor { "_" } else { "" }
                    ).unwrap();
                    write!(out, "{}{}", cursor::Goto(x, y + 1),
                        if state.tracks.get(&id.1).unwrap().monitor { "I" } else { "i" }
                    ).unwrap();
                },
                p_t: |action, id, state| match action {
                    Action::SelectB => Action::SoloTrack(
                        id.1,
                        !state.tracks.get(&id.1).unwrap().solo
                    ),
                    _ => Action::Noop,
                },

                y: VOID_RENDER,
                y_t: VOID_TRANSFORM,

                active: None,
            }
        ]);
    };

    let mut region_vec: Vec<(&u16, &Region)> = regions.iter().collect();
    region_vec.sort_by(|(_, a), (_, b)| a.offset.cmp(&b.offset));

    for (region_id, region) in region_vec.iter() {
        focii[region.track as usize].push(
            region::new(**region_id)
        )
    }

    return focii
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    let o1 = offset_char(1, state.sample_rate, state.tempo, state.zoom);
    TimelineState {
        tempo: match action.clone() {
            Action::Deselect => if let Some(t) = state.temp_tempo { t }
                else { state.tempo },
            _ => state.tempo
        },
        temp_tempo: match action.clone() {
            Action::SetTempo(t) => if t > 0 { Some(t) } else { Some(1) },
            _ => None
        },
        zoom: match action.clone() {
            Action::Deselect => if let Some(z) = state.temp_zoom { z }
                else { state.zoom },
            _ => state.zoom
        },
        temp_zoom: match action.clone() {
            Action::Zoom(z) => Some(z),
            _ => None
        },
        meter_beat: match action.clone() {
            Action::SetMeter(beat, _) => if beat > 0 { beat } else { 1 },
            _ => state.meter_beat,
        },
        meter_note: match action.clone() {
            Action::SetMeter(_, note) => if note > 0 { note } else { 1 },
            _ => state.meter_note,
        },
        loop_mode: match action.clone() {
            Action::LoopMode(on) => on,
            _ => state.loop_mode
        },
        seq_in: state.seq_in,
        seq_out: state.seq_out,
        loop_in: match action.clone() {
            Action::SetLoop(mark_in, mark_out) => if mark_in >= 0 { 
                if mark_in < mark_out { mark_in } else { state.loop_in }
            } else { 0 },
            _ => state.loop_in,
        },
        loop_out: match action.clone() {
            Action::SetLoop(mark_in, mark_out) => if mark_out >= 0 { 
                if mark_out > mark_in { mark_out } else { state.loop_out }
            } else { 0 },
            _ => state.loop_out,
        },
        sample_rate: state.sample_rate,
        tracks: {
            let mut new_tracks = state.tracks.clone();
            match action.clone() {
                Action::SoloTrack(id, is_on) => {
                    let track = new_tracks.get_mut(&id).unwrap();
                    track.solo = is_on;
                },
                Action::RecordTrack(id, is_on) => {
                    let track = new_tracks.get_mut(&id).unwrap();
                    track.record = is_on;
                },
                Action::MuteTrack(id, is_on) => {
                    let track = new_tracks.get_mut(&id).unwrap();
                    track.mute = is_on;
                },
                Action::MonitorTrack(id, is_on) => {
                    let track = new_tracks.get_mut(&id).unwrap();
                    track.monitor = is_on;
                }
                _ => {}
            };
            new_tracks
        },
        assets: match action.clone() {
            Action::AddRegion(_, _, asset_id, _, duration, src) => {
                let mut new_assets = state.assets.clone();
                new_assets.insert(asset_id, Asset {
                    src: src.clone(),
                    duration: duration.clone(),
                    channels: 2,
                    waveform: generate_partial_waveform(
                        src, 
                        duration, 
                        state.sample_rate, 
                        state.tempo, 
                        state.zoom
                    ),
                });
                new_assets
            },
            //Action::Zoom |
            Action::Deselect  => {
                let zoom = if let Some(z) = state.temp_zoom { z } else { state.zoom };
                let tempo = if let Some(t) = state.temp_tempo { t } else { state.tempo };
                if zoom != state.zoom || tempo != state.tempo {
                    let mut new_assets = state.assets.clone();
                    generate_waveforms(&mut new_assets, state.sample_rate, tempo, zoom);
                    new_assets
                } else {
                    state.assets.to_owned()
                }
            },
            _ => state.assets.to_owned()
        },
        regions: match action.clone() {
            Action::AddRegion(t_id, r_id, asset_id, offset, duration, src) => {
                let mut new_regions = state.regions.clone();
                new_regions.insert(r_id, Region {
                    asset_id,
                    asset_in: 0,
                    asset_out: duration,
                    offset: offset,
                    track: t_id,
                });
                new_regions
            },
            Action::MoveRegion(id, t_id, offset) => {
                let mut new_regions = state.regions.clone();
                let r = new_regions.get_mut(&id).unwrap();
                r.track = t_id;
                r.offset = offset;
                new_regions
            },
            _ => state.regions.clone(),
        },
        notes: match action.clone() {
            Action::AddNote(note) => {
                let mut new_notes = state.notes.clone();
                new_notes.push(note);
                new_notes
            },
            _ => state.notes.clone()
        },
        tick: match action.clone() {
            Action::Tick => !state.tick,
            _ => state.tick
        },
        playhead: match action {
            Action::Goto(o) => o,
            _ => state.playhead
        },
        scroll_x: match action {
            Action::Goto(o) => {
                let playhead_offset = char_offset(
                    o,
                    state.sample_rate,
                    state.tempo,
                    state.zoom);

                if playhead_offset > state.scroll_mid {
                    playhead_offset - state.scroll_mid
                } else { 0 }
            },
            _ => state.scroll_x
        },
        scroll_y: state.scroll_y,
        scroll_mid: state.scroll_mid,
        focus: state.focus,
    }
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, module: Element) -> Self {

        // Initialize State
        let mut initial_state: TimelineState = timeline::read(module); 

        generate_waveforms(&mut initial_state.assets, initial_state.sample_rate,
                           initial_state.tempo, initial_state.zoom);

        initial_state.scroll_mid = (width - REGIONS_X) / 2;

        Timeline {
            x,
            y,
            width,
            height,
            focii: generate_focii(&initial_state.tracks, &initial_state.regions),
            state: initial_state,
        }
    }
}

impl Layer for Timeline {
    fn render(&self, out: &mut Screen, target: bool) {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, false, !target);

        // Print track numbers
        for (id, track) in self.state.tracks.iter() {
            let track_y: u16 = win.y + 1 + TIMELINE_Y + (id*2) as u16;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(win.x+1, track_y),
                id).unwrap();
        }

        let playhead_offset = char_offset(
            self.state.playhead,
            self.state.sample_rate,
            self.state.tempo,
            self.state.zoom);

        // print tempo
        ruler::render(out, REGIONS_X, 6, 
            self.width-4,
            self.height,
            self.state.meter_beat,
            self.state.zoom,
            self.state.scroll_x,
            playhead_offset);

        roll::render(out,
            Window { 
                x: win.x + REGIONS_X, 
                y: win.y + TIMELINE_Y,
                w: win.w - REGIONS_X,
                h: win.h - TIMELINE_Y,
            },
            self.state.scroll_x.into(),
            self.state.sample_rate,
            self.state.tempo,
            self.state.zoom,
            &self.state.notes);

        if let Some(t) = self.state.temp_tempo {
            write!(out, "{}Generating waveforms...", cursor::Goto(
                win.x + (win.w / 2) - 10, win.y
            )).unwrap();
        }
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Let the focus transform the action 
        let multi_focus = &mut self.focii[self.state.focus.1][self.state.focus.0];
        let _action = multi_focus.transform(action.clone(), &mut self.state);

        self.state = reduce(self.state.clone(), _action.clone());

        // Actions which affect focii
        let (focus, default) = match _action.clone() {
            // Move focus to intersecting region on Tick
            Action::AddRegion(_, _, _, _, _, _) => {
                self.focii = generate_focii(&self.state.tracks, &self.state.regions);
                (self.state.focus, None)
            },
            Action::Goto(time) => match multi_focus.w_id.0 {
                FocusType::Region => {
                    // Find selected region within selected track
                    let mut new_focus = self.state.focus.0;
                    for (i, focus) in self.focii[self.state.focus.1].iter().enumerate() {
                        let region = self.state.regions.get(&focus.w_id.1).unwrap();
                        let duration = region.asset_out - region.asset_in;
                        if time >= region.offset && time < region.offset + duration {
                            new_focus = i;
                        }
                    }
                    ((new_focus, self.state.focus.1), Some(Action::Noop))
                },
                _ => (self.state.focus, Some(Action::Noop))
            },
            Action::Left => match multi_focus.w_id.0 {
                FocusType::Region => if self.state.playhead == 0 {
                    shift_focus(self.state.focus, &self.focii, Action::Left)
                } else {
                    (self.state.focus, Some(Action::Scrub(false)))
                }
                _ => shift_focus(self.state.focus, &self.focii, Action::Left),
            },
            Action::Right => match multi_focus.w_id.0 {
                FocusType::Region => (self.state.focus, Some(Action::Scrub(true))),
                _ => shift_focus(self.state.focus, &self.focii, Action::Right),
            },
            Action::Up | Action::Down => shift_focus(self.state.focus, &self.focii, _action.clone()),
            Action::Deselect => {
                // Get the global (white) ID of the current focus, generate a new focii
                // array based on the new tracks and regions, Then find the
                // focus that shares the ID of our current focus, and return 
                // that focus
                let current_id = self.focii[self.state.focus.1][self.state.focus.0].w_id.clone();
                match current_id.0 {
                    FocusType::Region => {
                        self.focii = generate_focii(&self.state.tracks, &self.state.regions);
                        let mut new_focus: (usize, usize) = self.state.focus;

                        'search: for (j, col) in self.focii.iter().enumerate() {
                            for (i, focus) in col.iter().enumerate() {
                                if focus.w_id == current_id {
                                    new_focus = (i,j);
                                    break 'search;
                                }
                            }
                        }
                        (new_focus, None)
                    },
                    _ => (self.state.focus, None)
                }
            },
            Action::Route => {
                let mut anchors = vec![];
                let mut counter = 0;
                let mut sorted_tracks: Vec<(&u16, &Track)> = self.state.tracks.iter().collect();
                sorted_tracks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (id, track) in sorted_tracks.iter() {
                    // Track output
                    anchors.push(Anchor {
                        name: format!("Track {}", *id),
                        index: counter,
                        module_id: 0,
                        input: false,
                    });
                    counter += 1;
                    // Track input
                    anchors.push(Anchor {
                        name: format!("Track {}", *id),
                        index: counter,
                        module_id: 0,
                        input: true,
                    });
                    counter += 1;
                }
                (self.state.focus, Some(Action::ShowAnchors(anchors)))
            }
            a @ Action::Zoom(_) |
            a @ Action::SetLoop(_,_) |
            a @ Action::LoopMode(_) |
            a @ Action::SetMeter(_,_) |
            a @ Action::SetTempo(_) |
            a @ Action::RecordTrack(_, _) |
            a @ Action::MuteTrack(_, _) |
            a @ Action::SoloTrack(_, _) |
            a @ Action::MonitorTrack(_, _) |
            a @ Action::Play |
            a @ Action::Stop  => (self.state.focus, Some(a)),
            _ => (self.state.focus, None)
        };

        self.state.focus = focus;

        match default {
            Some(a) => a,
            None => Action::Noop,
        }
    }
    fn undo(&mut self) {
        self.state = self.state.clone()
    }
    fn redo(&mut self) {
        self.state = self.state.clone()
    }
    fn alpha(&self) -> bool {
        false
    }
}
