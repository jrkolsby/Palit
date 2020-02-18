use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, MultiFocus, FocusType, ID, Window};
use crate::common::{char_offset, offset_char};
use crate::common::{REGIONS_X, TIMELINE_Y};
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
        r: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + 7 + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} TRIM ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }
        },
        r_t: void_transform,
        r_id: void_id.clone(),
        g: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} MOVE ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }
        },
        g_t: |action, id, state| match action {
            Action::Right => { 
                let r = state.midi_regions.get(&id.1).unwrap();
                let d_offset = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                Action::MoveRegion(id.1, r.track, r.offset + d_offset) 
            },
            Action::Left => { 
                let r = state.midi_regions.get(&id.1).unwrap();
                let d_offset = offset_char(1, state.sample_rate, state.tempo, state.zoom);
                Action::MoveRegion(id.1, r.track, r.offset - d_offset) 
            },
            _ => Action::Noop,
        },
        g_id: void_id.clone(),
        y: |mut out, window, id, state, focus| {
            if focus {
                let region = state.midi_regions.get(&id.1).unwrap();

                let region_offset = char_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + 14 + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} SPLIT ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }
        }, 

        y_t: void_transform,
        y_id: void_id.clone(),
        p: void_render, 
        p_t: void_transform,
        p_id: void_id.clone(),
        b: void_render, 
        b_t: void_transform,
        b_id: void_id.clone(),

        active: None,
    }
}