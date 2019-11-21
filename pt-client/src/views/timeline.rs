extern crate wavefile;

use wavefile::WaveFile;

use xmltree::Element;

use termion::{color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use std::io::{Write, Stdout};
use std::collections::HashMap;

use crate::components::{waveform, tempo, button, ruler};
use crate::common::{MultiFocus, render_focii, shift_focus, FocusType, Window, ID};
use crate::common::{Module, Action, Color, Asset, Region, Track};
use crate::common::{beat_offset, generate_waveforms};
use crate::modules::timeline;
use crate::views::{Layer};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

static MARGIN: (u16, u16) = (3, 3);
static TRACKS_X: u16 = 3;
static REGIONS_X: u16 = 16;
static TIMELINE_Y: u16 = 5;
static SCROLL_R: u16 = 40;
static SCROLL_L: u16 = 10;
static ASSET_PREFIX: &str = "storage/";

#[derive(Clone, Debug)]
pub struct TimelineState {
    // Requires XML write/read
    pub tempo: u16,
    pub time_beat: usize,
    pub time_note: usize,
    pub loop_mode: bool,
    pub seq_in: u32,
    pub seq_out: u32,
    pub loop_in: u32,
    pub loop_out: u32,
    pub sample_rate: u32,
    pub tracks: HashMap<u16, Track>,
    pub assets: HashMap<u16, Asset>,
    pub regions: HashMap<u16, Region>,

    // Ephemeral variables
    pub tick: bool,
    pub playhead: u16,
    pub zoom: usize,
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

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    TimelineState {
        tempo: state.tempo.clone(),
        time_beat: state.time_beat.clone(),
        time_note: state.time_note.clone(),
        loop_mode: state.loop_mode.clone(),
        seq_in: state.seq_in.clone(),
        seq_out: state.seq_out.clone(),
        loop_in: state.loop_in.clone(),
        loop_out: state.loop_out.clone(),
        sample_rate: state.sample_rate.clone(),
        tracks: state.tracks.clone(),
        assets: state.assets.clone(),
        regions: state.regions.clone(),

        tick: (state.playhead % 2) == 0,
        playhead: match action {
            Action::Tick => state.playhead + 1,
            Action::Right => state.playhead + 1,
            Action::Left => if state.playhead == 0 { 0 } else { state.playhead - 1 },
            _ => state.playhead.clone(),
        },
        zoom: state.zoom.clone(),
        scroll_x: state.scroll_x.clone(),
        scroll_y: state.scroll_y.clone(), 
        focus: state.focus.clone(),
    }
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, module: Element) -> Self {

        // Initialize State
        let mut initial_state: TimelineState = timeline::read(module); 
        generate_waveforms(&mut initial_state.assets, initial_state.sample_rate,
                           initial_state.tempo, initial_state.zoom);

        // Create empty select
        let void_id: ID = (FocusType::Void, 0);
        let void_render: fn(RawTerminal<Stdout>, Window, ID, &TimelineState) -> RawTerminal<Stdout> =
            |mut out, window, id, state| out;
        let void_transform: fn(Action, ID, &mut TimelineState) -> Action = 
            |action, id, state| action;

        let record_id: ID = (FocusType::Button, 0);
        let record_render: fn(RawTerminal<Stdout>, Window, ID, &TimelineState) -> RawTerminal<Stdout> = 
            |mut out, window, id, state| button::render(out, 2, 3, 56, "RECORD");
        let record_transform: fn(Action, ID, &mut TimelineState) -> Action = 
            |a, id, _| match a { Action::SelectR => Action::Record(id.1), _ => Action::Noop };

        /* TIMELINE LAYOUT
        Rec, 
        In, loop In, loop Out, Out

        */

        let mut focii: Vec<Vec<MultiFocus<TimelineState>>> = vec![vec![
            MultiFocus::<TimelineState> {
                r_id: record_id.clone(),
                r: record_render,
                r_t: |action, id, state| action,

                g_id: (FocusType::Button, 1),
                g: |mut out, window, id, state|
                    button::render(out, 60, 3, 19, "IMPORT"),
                g_t: |action, id, state| action,
                
                y_id: (FocusType::Param, 0),
                y: |mut out, window, id, state|
                    tempo::render(out, window.x+window.w-3, window.y,
                        state.time_beat,
                        state.time_note,
                        state.tempo,
                        state.tick),
                y_t: |action, id, state| action,

                p_id: (FocusType::Region, 0),
                p: |mut out, window, id, state| out,
                p_t: |action, id, state| action,

                b_id: (FocusType::Param, 2),
                b: |mut out, window, id, state| out,
                b_t: |action, id, state| action,

                active: None,
            }
        ]];

        let mut track_vec: Vec<(&u16, &Track)> = initial_state.tracks.iter().collect();
        track_vec.sort_by(|(_, a), (_, b)| a.index.cmp(&b.index));

        for (t_id, track) in track_vec.iter() {
            focii.push(vec![
                MultiFocus::<TimelineState> {
                    r_id: (FocusType::Button, **t_id),
                    g_id: (FocusType::Button, **t_id),
                    y_id: (FocusType::Button, **t_id),
                    p_id: (FocusType::Button, **t_id),
                    b_id: (FocusType::Button, **t_id),

                    r: |mut out, win, id, state|
                        button::render(out, win.x+TRACKS_X, win.y+TIMELINE_Y+2*id.1, 3, "R"),
                    r_t: |action, id, _| Action::Record(id.1),

                    g: |mut out, win, id, state|
                        button::render(out, win.x+TRACKS_X+4, win.y+TIMELINE_Y+(2*id.1), 3, "M"),
                    g_t: |action, id, _| Action::Mute(id.1),

                    b: |mut out, win, id, state|
                        button::render(out, win.x+TRACKS_X+8, win.y+TIMELINE_Y+(2*id.1), 3, "S"),
                    b_t: |action, id, _| Action::Solo(id.1),

                    p: |mut out, win, id, state| out,
                    p_t: |action, id, state| action,

                    y: |mut out, win, id, state| out,
                    y_t: |action, id, state| action,

                    active: None,
                }
            ]);
        };

        let mut region_vec: Vec<(&u16, &Region)> = initial_state.regions.iter().collect();
        region_vec.sort_by(|(_, a), (_, b)| a.offset.cmp(&b.offset));

        for (r_id, region) in region_vec.iter() {
            let (g, g_t, g_id): (fn(RawTerminal<Stdout>, 
                                    Window, ID, &TimelineState) -> RawTerminal<Stdout>,
                                    fn(Action, ID, &mut TimelineState) -> Action, ID) = (
                    |mut out, window, id, state| {
                        let region = state.regions.get(&id.1).unwrap();
                        let waveform = &state.assets.get(&region.asset_id).unwrap().waveform;

                        let asset_start_offset = beat_offset(
                            region.asset_in,
                            state.sample_rate,
                            state.tempo,
                            state.zoom,
                        ) as usize;

                        let asset_length_offset = beat_offset(
                            region.asset_out - region.asset_in,
                            state.sample_rate,
                            state.tempo,
                            state.zoom,
                        ) as usize;

                        let wave_slice = &waveform[asset_start_offset..(
                                                   asset_start_offset+asset_length_offset)];

                        let timeline_offset: u16 = state.scroll_x + 
                            beat_offset(region.offset, 
                                        state.sample_rate,
                                        state.tempo, 
                                        state.zoom) as u16;

                        let region_x = window.x + REGIONS_X + timeline_offset;
                        let region_y = window.y + 1 + TIMELINE_Y + 2 * region.track;

                        waveform::render(out, wave_slice, region_x, region_y)
                    },
                    |action, id, state| match action {
                        _ => Action::Noop,
                        Action::Right => { 
                            let mut region = state.regions.get_mut(&id.1).unwrap();
                            region.offset += (state.sample_rate/2);
                            Action::MoveRegion(id.1, 0, 0) 
                        }
                    },
                    (FocusType::Region, **r_id));

            // TODO: bin[1] bin[2] bin[3
            let (y, y_t, y_id) = (void_render, void_transform, void_id.clone());
            let (p, p_t, p_id) = (void_render, void_transform, void_id.clone());
            let (b, b_t, b_id) = (void_render, void_transform, void_id.clone());
            let (r, r_t, r_id) = (void_render, void_transform, void_id.clone());

            focii[region.track as usize].push(MultiFocus::<TimelineState> {
                r, r_t, r_id,
                g, g_t, g_id,
                y, y_t, y_id,
                p, p_t, p_id,
                b, b_t, b_id,

                active: None,
            })
        }

        Timeline {
            x,
            y,
            width,
            height,
            state: initial_state,
            focii,
        }
    }
}

impl Layer for Timeline {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        // Print track numbers
        for (id, track) in self.state.tracks.iter() {
            let track_y: u16 = win.y + 1 + TIMELINE_Y + (id*2) as u16;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(win.x+1, track_y),
                id).unwrap();
        }

        // print tempo
        out = ruler::render(out, REGIONS_X, 6, 
            self.width-4,
            self.height,
            self.state.time_beat,
            self.state.zoom,
            self.state.scroll_x,
            self.state.playhead);

        out = render_focii(out, win, self.state.focus.clone(), &self.focii, &self.state);

        write!(out, "{}", color::Bg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }
    fn dispatch(&mut self, action: Action) -> Action {

        // Let the focus transform the action 
        let multi_focus = &mut self.focii[self.state.focus.1][self.state.focus.0];
        let _action = multi_focus.transform(action.clone(), &mut self.state);

        // Intercept arrow actions to change focus or to return
        let (focus, default) = match action {
            Action::Up | Action::Down => shift_focus(self.state.focus, &self.focii, _action.clone()),
            // Only shift focus horizontally if playhead has exceeded current region
            Action::Left => match multi_focus.r_id.0 {
                FocusType::Region => {
                    let next_focus = &mut self.focii[self.state.focus.1][self.state.focus.0-1];
                    let region_id = next_focus.r_id.1;
                    let region = self.state.regions.get(&region_id).unwrap();
                    if region.offset <= self.state.playhead.into() {
                        shift_focus(self.state.focus, &self.focii, Action::Left)
                    } else {
                        (self.state.focus, None)
                    }
                },
                _ => shift_focus(self.state.focus, &self.focii, Action::Left),
            },
            Action::Right => match multi_focus.r_id.0 {
                FocusType::Region => {
                    if self.state.focus.0 == self.focii[self.state.focus.1].len()-1 {
                        (self.state.focus, None)
                    } else {
                        let next_focus = &mut self.focii[self.state.focus.1][self.state.focus.0+1];
                        let region_id = next_focus.r_id.1;
                        let region = self.state.regions.get(&region_id).unwrap();
                        if region.offset <= self.state.playhead.into() {
                            shift_focus(self.state.focus, &self.focii, Action::Right)
                        } else {
                            (self.state.focus, None)
                        }
                    }
                },
                _ => shift_focus(self.state.focus, &self.focii, Action::Right),
            },
            _ => (self.state.focus, None)
        };

        // Set focus, if the multifocus defaults, take no further action
        self.state.focus = focus;
        if let Some(_default) = default {
            match _default {
                Action::Up | Action::Down => return _default,
                _ => {}
            }
        }

        self.state = reduce(self.state.clone(), action);

        match _action {
            _ => Action::Noop
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
