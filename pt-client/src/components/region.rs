use std::io::{Write, Stdout};

use termion::{cursor};
use termion::raw::{RawTerminal};

use crate::common::{MultiFocus, FocusType, Action, ID, Window};
use crate::common::{beat_offset, offset_beat};
use crate::components::{waveform};
use crate::views::TimelineState;

use crate::common::{REGIONS_X, TIMELINE_Y};

pub fn new(region_id: u16) -> MultiFocus::<TimelineState> {

    let void_id: ID = (FocusType::Void, 0);
    let void_render: fn(RawTerminal<Stdout>, Window, ID, &TimelineState, bool) -> RawTerminal<Stdout> =
        |mut out, window, id, state, focus| out;
    let void_transform: fn(Action, ID, &mut TimelineState) -> Action = 
        |action, id, state| Action::Noop;

    MultiFocus::<TimelineState> {
        w_id: (FocusType::Region, region_id),
        w: |mut out, window, id, state, focus| {
            let region = state.regions.get(&id.1).unwrap();
            let waveform = &state.assets.get(&region.asset_id).unwrap().waveform;

            let region_offset = beat_offset(region.offset,
                state.sample_rate, state.tempo, state.zoom);

            let asset_start_offset = beat_offset(region.asset_in,
                state.sample_rate, state.tempo, state.zoom);

            let asset_length_offset = beat_offset(region.asset_out - region.asset_in,
                state.sample_rate, state.tempo, state.zoom);

            // Region appears to left of timeline
            if asset_length_offset + region_offset < state.scroll_x {
                return out;
            } 
            // Region appears to right of timeline
            else if region_offset > state.scroll_x + window.w {
                return out;
            } 

            // Region split by left edge of timeline
            let wave_in_i: usize = if region_offset < state.scroll_x {
                (state.scroll_x - region_offset) as usize
            // Left edge of region appears unclipped
            } else {
                asset_start_offset as usize
            };

            // Region split by right edge of timeline
            let wave_out_i: usize = if state.scroll_x + window.w < region_offset + asset_length_offset {
                (asset_start_offset + asset_length_offset) as usize - 
                (region_offset + asset_length_offset - (state.scroll_x + window.w)) as usize
            } else {
                (asset_start_offset + asset_length_offset) as usize
            };

            let wave_slice = &waveform[wave_in_i..wave_out_i];

            let timeline_offset = if region_offset >= state.scroll_x {
                region_offset - state.scroll_x
            } else { 0 };

            let region_x = window.x + REGIONS_X + timeline_offset;
            let region_y = window.y + 1 + TIMELINE_Y + 2 * region.track;

            waveform::render(out, wave_slice, region_x, region_y)
        }, 
        r: |mut out, window, id, state, focus| {
            if focus {
                let region = state.regions.get(&id.1).unwrap();

                let region_offset = beat_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + 7 + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} TRIM ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }
            out
        },
        r_t: void_transform,
        r_id: void_id.clone(),
        g: |mut out, window, id, state, focus| {
            if focus {
                let region = state.regions.get(&id.1).unwrap();

                let region_offset = beat_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} MOVE ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }

            out
        },
        g_t: |action, id, state| match action {
            Action::Right => { 
                let r = state.regions.get(&id.1).unwrap();
                let d_offset = offset_beat(1, state.sample_rate, state.tempo, state.zoom);
                Action::MoveRegion(id.1, r.track, r.offset+d_offset) 
            },
            Action::Left => { 
                let r = state.regions.get(&id.1).unwrap();
                let d_offset = offset_beat(1, state.sample_rate, state.tempo, state.zoom);
                Action::MoveRegion(id.1, r.track, r.offset-d_offset) 
            },
            Action::Up => { 
                let r = state.regions.get(&id.1).unwrap();
                Action::MoveRegion(id.1, r.track-1, r.offset) 
            },
            Action::Down => { 
                let r = state.regions.get(&id.1).unwrap();
                Action::MoveRegion(id.1, r.track+1, r.offset) 
            },
            _ => Action::Noop,
        },
        g_id: void_id.clone(),
        y: |mut out, window, id, state, focus| {
            if focus {
                let region = state.regions.get(&id.1).unwrap();

                let region_offset = beat_offset(region.offset,
                    state.sample_rate, state.tempo, state.zoom);

                let timeline_offset = if region_offset >= state.scroll_x {
                    region_offset - state.scroll_x
                } else { 0 };

                let region_x = window.x + 14 + REGIONS_X + timeline_offset;
                let region_y = window.y + 2 + TIMELINE_Y + 2 * region.track;

                write!(out, "{} SPLIT ",
                    cursor::Goto(region_x, region_y)).unwrap();
            }
            out
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