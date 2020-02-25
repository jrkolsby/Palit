use std::io::{Write, Stdout};
use std::collections::HashMap;

use xmltree::Element;
use termion::cursor;
use libcommon::{Action, Anchor, Note, Param, Offset};

use crate::components::{button, ruler, roll};
use crate::components::{region_midi, track_header, region_audio, timeline_meter, timeline_nav};
use crate::common::{ID, VOID_ID, FocusType};
use crate::common::{MultiFocus, render_focii, shift_focus, generate_partial_waveform};
use crate::common::{Screen, Asset, AudioRegion, MidiRegion, Track, Window, REGIONS_PER_TRACK};
use crate::common::{char_offset, offset_char, generate_waveforms};
use crate::modules::timeline;
use crate::views::{Layer};

use crate::common::{REGIONS_X, TIMELINE_Y};


#[derive(Clone, Debug)]
pub struct TimelineState {
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
    pub regions: HashMap<u16, AudioRegion>,
    pub midi_regions: HashMap<u16, MidiRegion>,

    // Ephemeral variables
    pub tick: bool,
    pub playhead: u32,
    pub scroll_mid: u16,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub focus: (usize, usize),
}

// Properties which aren't changed on every action
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    pub state: TimelineState,
    focii: Vec<Vec<MultiFocus<TimelineState>>>,
}

fn generate_focii(tracks: &HashMap<u16, Track>, 
                  audio_regions: &HashMap<u16, AudioRegion>,
                  midi_regions: &HashMap<u16, MidiRegion>) -> Vec<Vec<MultiFocus<TimelineState>>> {
    // Push header navigation and meter / zoom controls to first row
    let mut focii: Vec<Vec<MultiFocus<TimelineState>>> = vec![vec![
        timeline_nav::new(),
        timeline_meter::new()
    ]];

    let mut track_vec: Vec<(&u16, &Track)> = tracks.iter().collect();
    track_vec.sort_by(|(_, a), (_, b)| a.index.cmp(&b.index));

    // Push track headers onto new rows for each track. 
    // ... There must be at least one focus present on
    // ... each track or else DelRegion will panic
    for (t_id, track) in track_vec.iter() {
        focii.push(vec![track_header::new(**t_id)]);
    };

    // Push audio and midi regions to their track vector
    let mut sorted_audio_regions: Vec<(&u16, &AudioRegion)> = audio_regions.iter().collect();
    sorted_audio_regions.sort_by(|(_, a), (_, b)| a.offset.cmp(&b.offset));

    let mut sorted_midi_regions: Vec<(&u16, &MidiRegion)> = midi_regions.iter().collect();
    sorted_midi_regions.sort_by(|(_, a), (_, b)| a.offset.cmp(&b.offset));

    // Track current position in both arrays and push with global offset ordering
    let mut audio_i: usize = 0;
    let mut midi_i: usize = 0;

    loop {
        let maybe_audio = sorted_audio_regions.get(audio_i);
        let maybe_midi = sorted_midi_regions.get(midi_i);

        if maybe_audio.is_some() && maybe_midi.is_some() {
            let (audio_r_id, audio_region) = maybe_audio.unwrap();
            let (midi_r_id, midi_region) = maybe_midi.unwrap();

            if (audio_region.offset < midi_region.offset) {
                focii[audio_region.track as usize].push(
                    region_audio::new(**audio_r_id)
                );
                audio_i += 1;
            } else {
                focii[midi_region.track as usize].push(
                    region_midi::new(**midi_r_id)
                );
                midi_i += 1;
            }
            continue;
        }
        if maybe_audio.is_some() {
            let (audio_r_id, audio_region) = maybe_audio.unwrap();
            focii[audio_region.track as usize].push(
                region_audio::new(**audio_r_id)
            );
            audio_i += 1;
        } else if maybe_midi.is_some() {
            let (midi_r_id, midi_region) = maybe_midi.unwrap();
            focii[midi_region.track as usize].push(
                region_midi::new(**midi_r_id)
            );
            midi_i += 1;
        } else {
            // No more regions of any type
            break;
        }
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
            Action::AddRegion(_, _, asset_id, _, duration, asset_in, src) => {
                let mut new_assets = state.assets.clone();
                if let Some(mut old_asset) = new_assets.get_mut(&asset_id) {
                    // Do not clip assets, only add to them
                    if old_asset.duration < duration {
                        old_asset.duration = duration.clone();
                        old_asset.waveform = generate_partial_waveform(
                            src, 
                            duration, 
                            state.sample_rate, 
                            state.tempo, 
                            state.zoom
                        );
                    }
                } else {
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
                }
                new_assets
            },
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
            Action::AddRegion(t_id, r_id, asset_id, offset, duration, asset_in, src) => {
                let mut new_regions = state.regions.clone();
                // Because each tape module is an isolated instance, it can only generate
                // ... region id's unique to its own scope. We need a global ID to store
                // ... in the timeline, so we must limit the number of regions per track 
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                new_regions.insert(global_id, AudioRegion {
                    asset_id,
                    asset_in,
                    duration,
                    offset,
                    track: t_id,
                });
                new_regions
            },
            Action::MoveRegion(t_id, r_id, offset) => {
                let mut new_regions = state.regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                if let Some(mut r) = new_regions.get_mut(&global_id) {
                    r.offset = offset;
                }
                new_regions
            },
            Action::SplitRegion(t_id, r_id, _) |
            Action::DelRegion(t_id, r_id) => {
                let mut new_regions = state.regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                new_regions.remove(&global_id);
                new_regions
            },
            _ => state.regions.clone(),
        },
        midi_regions: match action.clone() {
            Action::AddNote(t_id, note) => {
                let mut new_regions = state.midi_regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + note.r_id;
                if let Some(mut region) = new_regions.get_mut(&global_id){ 
                    region.notes.push(note.clone()) 
                };
                new_regions
            },
            Action::MoveRegion(t_id, r_id, offset) => {
                let mut new_regions = state.midi_regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                if let Some(mut r) = new_regions.get_mut(&global_id) {
                    let diff: i32 = offset as i32 - r.offset as i32;
                    for mut note in r.notes.iter_mut() {
                        note.t_in = (note.t_in as i32 + diff) as Offset;
                        note.t_out = (note.t_out as i32 + diff) as Offset;
                    }
                    r.offset = offset;
                }
                new_regions
            },
            Action::AddMidiRegion(t_id, r_id, offset, duration) => {
                let mut new_regions = state.midi_regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                // If we try adding a midi region which already exists, it's because
                // ... it just finished recording, so update duration 
                if let Some(old_region) = new_regions.get_mut(&global_id) {
                    old_region.duration = duration;
                    old_region.offset = offset;
                } else {
                    new_regions.insert(global_id, MidiRegion {
                        duration,
                        offset,
                        track: t_id,
                        notes: vec![]
                    });
                }
                new_regions
            },
            Action::SplitRegion(t_id, r_id, _) |
            Action::DelRegion(t_id, r_id) => {
                let mut new_regions = state.midi_regions.clone();
                let global_id = t_id * REGIONS_PER_TRACK + r_id;
                new_regions.remove(&global_id);
                new_regions
            },
            _ => state.midi_regions
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
            Action::Deselect => {
                if let Some(z) = state.temp_zoom { 
                    let playhead_offset = char_offset(
                        state.playhead,
                        state.sample_rate,
                        state.tempo,
                        z);

                    if playhead_offset > state.scroll_mid {
                        playhead_offset - state.scroll_mid
                    } else { 0 }
                } else {
                    state.scroll_x
                }
            },
            Action::Goto(o) => {
                let playhead_offset = char_offset(
                    o,
                    state.sample_rate,
                    state.tempo,
                    state.zoom
                );

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
            focii: generate_focii(
                &initial_state.tracks, 
                &initial_state.regions, 
                &initial_state.midi_regions),
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
            // Move focus to the left when a region is deleted
            a @ Action::DelRegion(_,_) |
            a @ Action::SplitRegion(_,_,_) => {
                self.focii = generate_focii(
                    &self.state.tracks, 
                    &self.state.regions, 
                    &self.state.midi_regions);
                ((self.state.focus.0 - 1, self.state.focus.1), Some(a))
            },
            // Regenerate to make new regions visible and default
            a @ Action::LoopRegion(_,_) => {
                self.focii = generate_focii(
                    &self.state.tracks, 
                    &self.state.regions, 
                    &self.state.midi_regions);
                (self.state.focus, Some(a))
            },
            // Regenerate to make new regions visible 
            Action::AddMidiRegion(_, _, _, _) |
            Action::AddRegion(_, _, _, _, _, _, _) => {
                self.focii = generate_focii(
                    &self.state.tracks, 
                    &self.state.regions, 
                    &self.state.midi_regions);
                (self.state.focus, None)
            },
            Action::Goto(time) => match multi_focus.w_id.0 {
                FocusType::Region => {
                    // Find selected region within selected track
                    let mut new_focus = self.state.focus.0;
                    for (i, focus) in self.focii[self.state.focus.1].iter().enumerate() {
                        // This id could be a midi region or an audio region
                        if let Some(audio_region) = self.state.regions.get(&focus.w_id.1) {
                            if time >= audio_region.offset && time < audio_region.offset + audio_region.duration {
                                new_focus = i;
                            }
                        }
                        if let Some(midi_region) = self.state.midi_regions.get(&focus.w_id.1) {
                            if time >= midi_region.offset && time < midi_region.offset + midi_region.duration {
                                new_focus = i;
                            }
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
                        self.focii = generate_focii(
                            &self.state.tracks, 
                            &self.state.regions, 
                            &self.state.midi_regions);
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
            a @ Action::MoveRegion(_,_,_) |
            a @ Action::Zoom(_) |
            a @ Action::SetLoop(_,_) |
            a @ Action::LoopMode(_) |
            a @ Action::SetMeter(_,_) |
            a @ Action::SetTempo(_) |
            a @ Action::RecordTrack(_, _) |
            a @ Action::MuteTrack(_, _) |
            a @ Action::SoloTrack(_, _) |
            a @ Action::MonitorTrack(_, _) |
            a @ Action::Record |
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
    fn alpha(&self) -> bool { false }
    fn save(&self) -> Option<Element> { 
        Some(timeline::write(self.state.clone()))
    }
}
