extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use std::io::{Write, Stdout};
use std::collections::HashMap;
use std::convert::TryInto;

use crate::components::{waveform, tempo, button, ruler};
use crate::common::{MultiFocus, render_focii, shift_focus, FocusType, Window, ID};
use crate::common::{Action, Color, TimelineState, Region};
use crate::common::{read_document, beat_offset, file_to_pairs};
use crate::views::{Layer};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

static MARGIN: (u16, u16) = (3, 3);
static EXTRAS_W: u16 = 6;
static EXTRAS_H: u16 = 7;
static SCROLL_R: u16 = 40;
static SCROLL_L: u16 = 10;
static ASSET_PREFIX: &str = "storage/";

// STATIC PROPERTIES THROUGHOUT VIEW'S LIFETIME
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    project_src: String,
    state: TimelineState,
    focii: Vec<Vec<MultiFocus<TimelineState>>>,
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    let playhead = match action {
        Action::Tick => state.playhead + 1,
        Action::Right => state.playhead + 1,
        Action::Left => if state.playhead == 0 { 0 } else { state.playhead - 1 },
        _ => state.playhead.clone(),
    };
    TimelineState {
        name: state.name.clone(),
        tempo: state.tempo.clone(),
        time_beat: state.time_beat.clone(),
        time_note: state.time_note.clone(),
        sample_rate: state.sample_rate.clone(),
        zoom: state.zoom.clone(),
        assets: state.assets.clone(),
        sequence: state.sequence.clone(),
        loop_mode: state.loop_mode.clone(),
        scroll_x: state.scroll_x.clone(),
        scroll_y: state.scroll_y.clone(), 
        tick: (playhead % 2) == 0,
        duration_beat: state.duration_beat.clone(),
        duration_measure: state.duration_measure.clone(),
        playhead,
        focus: state.focus.clone(),
        // We don't want to necessarily clone this because of the overhead, 
        // as long as we never remove from it we should be able to implement
        // undo / redo.
        waveforms: state.waveforms.to_owned(), 
        regions: state.regions.to_owned(),
    }
}

fn generate_waveforms(state: &TimelineState) 
    -> HashMap<u16, Vec<(u8, u8)>> {
    let mut waveforms: HashMap<u16, Vec<(u8, u8)>> = HashMap::new();

    for asset in state.assets.iter() {
        eprintln!("generating wave: {}", asset.src.clone());

        let asset_file = WaveFile::open(asset.src.clone()).unwrap();

        let num_pairs: usize = beat_offset(
            asset.duration,
            state.sample_rate.clone(),
            state.tempo,
            state.zoom) as usize;

        eprintln!("num {}", num_pairs);

        let pairs: Vec<(u8, u8)> = file_to_pairs(asset_file, num_pairs, 4);

        waveforms.insert(asset.id, pairs);
    }
    waveforms
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, project_src: String) -> Self {

        // Initialize State
        let mut initial_state: TimelineState = read_document(project_src.clone()); 
        initial_state.waveforms = generate_waveforms(&initial_state);

        let void_id: ID = (FocusType::Void, 0);
        let void_render: fn(RawTerminal<Stdout>, Window, ID, &TimelineState) -> RawTerminal<Stdout> =
            |mut out, window, id, state| out;
        let void_transform: fn(Action, ID, &mut TimelineState) -> Action = 
            |action, id, state| action;

        let record_id: ID = (FocusType::Button, 0);
        let record_render: fn(RawTerminal<Stdout>, Window, ID, &TimelineState) -> RawTerminal<Stdout> = 
            |mut out, window, id, state| button::render(out, 2, 3, 56, "RECORD");
        let record_transform: fn(Action, ID, &mut TimelineState) -> Action = 
            |a, _, _| match a { Action::SelectR => Action::Record, _ => Action::Noop };

        // We're gonna fill this up with regions starting at the top left-hand corner, each inner vector
        // should have at most 4 Some's and be exactly 4 in length. 
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
                        state.duration_measure,
                        state.duration_beat,
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

        // Populate focii with regions
        // TODO: Stagger entries between two tracks
        for (i, track) in initial_state.sequence.iter().enumerate() {
            focii.push(vec![]);
            for (j, region) in track.regions.iter().enumerate() {
                let (g, g_t, g_id): (fn(RawTerminal<Stdout>, 
                                     Window, ID, &TimelineState) -> RawTerminal<Stdout>,
                                     fn(Action, ID, &mut TimelineState) -> Action, ID) = (
                        |mut out, window, id, state| {
                            let region = &state.regions.get(&id.1).unwrap();
                            let waveform = &state.waveforms[&region.asset_id];
                            let offset: u16 = state.scroll_x + 
                                beat_offset(region.offset, 
                                            state.sample_rate.clone(), 
                                            state.tempo, 
                                            state.zoom) as u16;
                            let region_x = window.x + EXTRAS_W + offset;
                            let region_y = window.y + EXTRAS_H + 2 * region.track;
                            waveform::render(out, waveform, region_x, region_y)
                        },
                        |action, id, state| match action {
                            _ => Action::Noop,
                            Action::Right => { Action::MoveRegion(id.1, 0, 0) }
                        },
                        (FocusType::Region, region.id.try_into().unwrap()));

                // TODO: bin[1] bin[2] bin[3
                let (y, y_t, y_id) = (void_render, void_transform, void_id.clone());
                let (p, p_t, p_id) = (void_render, void_transform, void_id.clone());
                let (b, b_t, b_id) = (void_render, void_transform, void_id.clone());

                focii.last_mut().unwrap().push(MultiFocus::<TimelineState> {
                    r_id: record_id.clone(),
                    r: record_render,
                    r_t: record_transform,

                    g, g_t, g_id,
                    y, y_t, y_id,
                    p, p_t, p_id,
                    b, b_t, b_id,

                    active: None,
                })
            }
        }

        Timeline {
            x,
            y,
            width,
            height,
            project_src,
            state: initial_state,
            focii,
        }
    }
}

impl Layer for Timeline {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        // Print track sidebar
        for (i, _track) in self.state.sequence.iter().enumerate() {
            let track_y: u16 = self.y + EXTRAS_H + (i*2) as u16;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(self.x+1, track_y),
                i+1).unwrap();
        }

        let name_len: u16 = self.state.name.len() as u16;
        let name_x: u16 = self.x + (self.width/2) - (name_len/2);
        write!(out, "{} {} ",
            cursor::Goto(name_x,self.y),
            self.state.name).unwrap();

        // print tempo
        out = ruler::render(out, 5, 6, 
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

        // Intercept arrow actions to change focus
        let (focus, default) = shift_focus(self.state.focus, &self.focii, _action.clone());

        // Set focus, if the multifocus defaults, take no further action
        self.state.focus = focus;
        if let Some(_default) = default {
            return _default;
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
