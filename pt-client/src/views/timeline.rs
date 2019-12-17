extern crate wavefile;

use xmltree::Element;

use termion::cursor;
use termion::raw::{RawTerminal};

use std::io::{Write, Stdout};
use std::collections::HashMap;

use crate::components::{tempo, button, ruler, region};
use crate::common::{ID, VOID_ID, FocusType};
use crate::common::{MultiFocus, render_focii, shift_focus };
use crate::common::{Action, Asset, Region, Track, Anchor, Window};
use crate::common::{beat_offset, offset_beat, generate_waveforms};
use crate::modules::timeline;
use crate::views::{Layer};

use crate::common::{REGIONS_X, TIMELINE_Y};

static TRACKS_X: u16 = 3;
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
    pub playhead: u32,
    pub scroll_x: u16,
    pub zoom: usize,
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

static VOID_RENDER: fn( RawTerminal<Stdout>, 
        Window, ID, &TimelineState, bool) -> RawTerminal<Stdout> =
    |mut out, window, id, state, focus| out;
static VOID_TRANSFORM: fn(Action, ID, &mut TimelineState) -> Action = 
    |action, id, state| Action::Noop;

fn generate_focii(tracks: &HashMap<u16, Track>, 
                  regions: &HashMap<u16, Region>) -> Vec<Vec<MultiFocus<TimelineState>>> {
    let mut focii: Vec<Vec<MultiFocus<TimelineState>>> = vec![vec![
        MultiFocus::<TimelineState> {
            w: |mut out, window, id, state, focus| out,
            w_id: VOID_ID.clone(),

            r_id: (FocusType::Button, 0),
            r: |mut out, window, id, state, focus| 
                button::render(out, 2, 2, 20, "RECORD"),
            r_t: |a, id, _| match a { 
                Action::SelectR => Action::RecordAt(id.1), 
                _ => Action::Noop 
            },

            g_id: (FocusType::Button, 1),
            g: |mut out, window, id, state, focus|
                button::render(out, 24, 2, 19, "IMPORT"),
            g_t: |action, id, state| action,
            
            y_id: (FocusType::Param, 0),
            y: |mut out, window, id, state, focus|
                tempo::render(out, window.x+window.w-3, window.y,
                    state.time_beat,
                    state.time_note,
                    state.tempo,
                    state.tick),
            y_t: |action, id, state| action,

            p_id: (FocusType::Region, 0),
            p: |mut out, window, id, state, focus| out,
            p_t: |action, id, state| action,

            b_id: (FocusType::Param, 2),
            b: |mut out, window, id, state, focus| out,
            b_t: |action, id, state| action,

            active: None,
        }
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

                r: |mut out, win, id, state, focus|
                    button::render(out, win.x+TRACKS_X, win.y+TIMELINE_Y+2*id.1, 3, "R"),
                r_t: |action, id, _| Action::RecordAt(id.1),

                g: |mut out, win, id, state, focus|
                    button::render(out, win.x+TRACKS_X+4, win.y+TIMELINE_Y+(2*id.1), 3, "M"),
                g_t: |action, id, _| Action::MuteAt(id.1),

                b: |mut out, win, id, state, focus|
                    button::render(out, win.x+TRACKS_X+8, win.y+TIMELINE_Y+(2*id.1), 3, "S"),
                b_t: |action, id, _| Action::SoloAt(id.1),

                p: |mut out, win, id, state, focus| out,
                p_t: |action, id, state| Action::Noop,

                y: |mut out, win, id, state, focus| out,
                y_t: |action, id, state| Action::Noop,

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
    let o1 = offset_beat(1, state.sample_rate, state.tempo, state.zoom);
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
        regions: match action {
            Action::MoveRegion(id, t_id, offset) => {
                let mut new_regions = state.regions.clone();
                let r = new_regions.get_mut(&id).unwrap();
                r.track = t_id;
                r.offset = offset;
                new_regions
            },
            _ => state.regions.clone(),
        },
        tick: (state.playhead % 2) == 0,
        playhead: match action {
            Action::Tick => state.playhead + o1,
            Action::Right => state.playhead + o1,
            Action::Left => if state.playhead < o1 { 0 } 
                else { state.playhead - o1 },
            _ => state.playhead.clone(),
        },
        zoom: state.zoom.clone(),
        scroll_x: {
            let playhead_offset = beat_offset(
                state.playhead,
                state.sample_rate,
                state.tempo,
                state.zoom);

            match action {
                Action::Left => 
                    if playhead_offset > 0 && state.scroll_x > 0 && 
                        playhead_offset < state.scroll_x + SCROLL_L {
                        state.scroll_x-1
                    } else { 
                        state.scroll_x.clone() 
                    },
                Action::Right => 
                    if playhead_offset > state.scroll_x + SCROLL_R {
                        state.scroll_x+1
                    } else {
                        state.scroll_x.clone()
                    },
                _ => state.scroll_x.clone(),
            }
        },
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
    fn render(&self, mut out: RawTerminal<Stdout>, target: bool) -> RawTerminal<Stdout> {

        let win: Window = Window { x: self.x, y: self.y, h: self.height, w: self.width };

        out = render_focii(
            out, win, 
            self.state.focus.clone(), 
            &self.focii, &self.state, !target);

        // Print track numbers
        for (id, track) in self.state.tracks.iter() {
            let track_y: u16 = win.y + 1 + TIMELINE_Y + (id*2) as u16;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(win.x+1, track_y),
                id).unwrap();
        }

        let playhead_offset = beat_offset(
            self.state.playhead,
            self.state.sample_rate,
            self.state.tempo,
            self.state.zoom);

        // print tempo
        out = ruler::render(out, REGIONS_X, 6, 
            self.width-4,
            self.height,
            self.state.time_beat,
            self.state.zoom,
            self.state.scroll_x,
            playhead_offset);

        out.flush().unwrap();
        out
    }

    fn dispatch(&mut self, action: Action) -> Action {

        // Let the focus transform the action 
        let multi_focus = &mut self.focii[self.state.focus.1][self.state.focus.0];
        let _action = multi_focus.transform(action.clone(), &mut self.state);

        self.state = reduce(self.state.clone(), _action.clone());
        
        // Intercept arrow actions to change focus or to return
        let (focus, default) = match _action {
            // Only shift focus horizontally if playhead has exceeded current region
            Action::Left => match multi_focus.w_id.0 {
                FocusType::Region => {
                    let region = self.state.regions.get(&multi_focus.w_id.1).unwrap();

                    if self.state.playhead <= region.offset  {
                        shift_focus(self.state.focus, &self.focii, Action::Left)
                    } else {
                        (self.state.focus, None)
                    }
                },
                _ => shift_focus(self.state.focus, &self.focii, Action::Left),
            },
            Action::Right => match multi_focus.w_id.0 {
                FocusType::Region => {
                    if self.state.focus.0 == self.focii[self.state.focus.1].len()-1 {
                        (self.state.focus, None)
                    } else {
                        let next_focus = &mut self.focii[self.state.focus.1][self.state.focus.0+1];
                        let next_region = self.state.regions.get(&next_focus.w_id.1).unwrap();

                        if self.state.playhead >= next_region.offset {
                            shift_focus(self.state.focus, &self.focii, Action::Right)
                        } else {
                            (self.state.focus, None)
                        }
                    }
                },
                _ => shift_focus(self.state.focus, &self.focii, Action::Right),
            },
            Action::Up | Action::Down => shift_focus(self.state.focus, &self.focii, _action.clone()),
            Action::Deselect => {
                // Get the global (white) ID of the current focus, generate a new focii
                // array based on the new tracks and regions, Then find the
                // focus that shares the ID of our current focus, and return 
                // that focus
                let current_id = self.focii[self.state.focus.1][self.state.focus.0].w_id.clone();
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
            Action::Route => {
                let mut anchors = vec![];
                for (id, track) in self.state.tracks.iter() {
                    // Track output
                    eprintln!("ID{}", id);
                    anchors.push(Anchor {
                        name: format!("Out {}", *id),
                        id: (*id-1) * 2,
                        module_id: 0,
                        x: TRACKS_X ,
                        y: TIMELINE_Y + 2 + 2 * id,
                        input: false,
                    });
                    // Track input
                    anchors.push(Anchor {
                        name: format!("In {}", *id),
                        id: (*id-1) * 2 + 1, 
                        module_id: 0,
                        x: TRACKS_X + 1,
                        y: TIMELINE_Y + 3 + 2 * id,
                        input: true,
                    });
                }
                eprintln!("{:?}", anchors);
                (self.state.focus, Some(Action::ShowAnchors(anchors)))
            }
            _ => (self.state.focus, None)
        };

        self.state.focus = focus;

        match default {
            Some(a) => a,
            None => Action::Noop
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
    fn shift(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
