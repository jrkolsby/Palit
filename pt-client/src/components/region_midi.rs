use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, MultiFocus, FocusType, ID, Window};
use crate::common::{char_offset, offset_char};
use crate::common::{REGIONS_X, TIMELINE_Y, REGIONS_PER_TRACK};
use crate::components::{waveform, roll};
use crate::views::TimelineState;

pub fn new(region_id: u16) -> MultiFocus::<TimelineState> {

    let void_id: ID = (FocusType::Void, 0);
    let void_render: fn(&mut Screen, Window, ID, &TimelineState, bool) =
        |_, _, _, _, _| {};
    let void_transform: fn(Action, ID, &TimelineState) -> Action = 
        |a, _, _| a;

    MultiFocus::<TimelineState> {
        w_id: (FocusType::Region, region_id),
        w: |mut out, window, id, state, focus| {
            let region = state.midi_regions.get(&id.1).unwrap();

            roll::render(out,
                Window { 
                    x: window.x + REGIONS_X, 
                    y: window.y + TIMELINE_Y,
                    w: window.w - REGIONS_X,
                    h: window.h - TIMELINE_Y,
                },
                state.scroll_x.into(),
                state.sample_rate,
                state.tempo,
                state.zoom,
                &region.notes);
        }, 

        r_id: void_id.clone(),
        r_t: |action, id, state| match action {
            Action::SelectR => {
                let local_id = id.1 % REGIONS_PER_TRACK;
                let track_id = id.1 / REGIONS_PER_TRACK;
                Action::LoopRegion(track_id, local_id)
            },
            a @ Action::AddMidiRegion(_,_,_,_) => a,
            _ => Action::Noop,
        },
        r: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let label_x = window.x + 15 + REGIONS_X + timeline_offset;
                let label_y = window.y + 2 + TIMELINE_Y + (2 * region.track);

                write!(out, "{} LOOP ",
                    cursor::Goto(label_x, label_y)).unwrap();
            }
        },

        g_id: void_id.clone(),
        g_t: |action, id, state| match action {
            Action::Right => { 
                let r = state.midi_regions.get(&id.1).unwrap();
                let d_offset = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                let local_id = id.1 % REGIONS_PER_TRACK;
                Action::MoveRegion(r.track, local_id, r.offset + d_offset) 
            },
            Action::Left => { 
                let r = state.midi_regions.get(&id.1).unwrap();
                let d_offset = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                let local_id = id.1 % REGIONS_PER_TRACK;
                Action::MoveRegion(r.track, local_id,
                    if r.offset < d_offset { 0 } else { r.offset - d_offset })  
            },
            _ => Action::Noop,
        },
        g: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let label_x = window.x + REGIONS_X + timeline_offset;
                let label_y = window.y + 2 + TIMELINE_Y + (2 * region.track);

                write!(out, "{} MOVE ",
                    cursor::Goto(label_x, label_y)).unwrap();
            }
        },

        y_id: void_id.clone(),
        y_t: |action, id, state| match action {
            Action::SelectY => {
                let local_id = id.1 % REGIONS_PER_TRACK;
                let track_id = id.1 / REGIONS_PER_TRACK;
                Action::SplitRegion(track_id, local_id, state.playhead)
            },
            a @ Action::AddMidiRegion(_,_,_,_) |
            a @ Action::AddNote(_,_) => a,
            _ => Action::Noop
        },
        y: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let label_x = window.x + 7 + REGIONS_X + timeline_offset;
                let label_y = window.y + 2 + TIMELINE_Y + (2 * region.track);

                write!(out, "{} SPLIT ",
                    cursor::Goto(label_x, label_y)).unwrap();
            }
        }, 

        p_id: void_id.clone(),
        p_t: |action, id, state| match action {
            Action::SelectP => {
                let local_id = id.1 % REGIONS_PER_TRACK;
                let track_id = id.1 / REGIONS_PER_TRACK;
                Action::DelRegion(track_id, local_id)
            },
            _ => Action::Noop
        },
        p: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let label_x = window.x + 22 + REGIONS_X + timeline_offset;
                let label_y = window.y + 2 + TIMELINE_Y + (2 * region.track);

                write!(out, "{} DEL ",
                    cursor::Goto(label_x, label_y)).unwrap();
            }
        }, 

        b_id: void_id.clone(),
        b_t: void_transform,
        b: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let label_x = window.x + 28 + REGIONS_X + timeline_offset;
                let label_y = window.y + 2 + TIMELINE_Y + (2 * region.track);

                write!(out, "{} EDIT ",
                    cursor::Goto(label_x, label_y)).unwrap();
            }
        }, 

        active: None,
    }
}