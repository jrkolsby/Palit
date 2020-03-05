use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, MultiFocus, FocusType, ID, Window};
use crate::components::{tempo};
use crate::views::TimelineState;

pub fn new() -> MultiFocus::<TimelineState> {

    let void_id: ID = (FocusType::Void, 0);
    let void_render: fn(&mut Screen, Window, ID, &TimelineState, bool) =
        |_, _, _, _, _| {};
    let void_transform: fn(Action, ID, &TimelineState) -> Action = 
        |a, _, _| a;

    MultiFocus::<TimelineState> {
        w_id: void_id.clone(),
        w: void_render,

        b_id: void_id.clone(),
        b_t: void_transform,
        b: void_render,

        y_id: (FocusType::Param, 0),
        y_t: |a, id, state| {
            let zoom = if let Some(z) = state.temp_zoom { z } else { state.zoom };
            match a {
                Action::Up => Action::Zoom(zoom + 1),
                Action::Down => {
                    if zoom > 1 {
                        Action::Zoom(zoom - 1)
                    } else {
                        Action::Noop
                    }
                },
                _ => Action::Noop,
            }
        },
        y: |out, window, id, state, focus| {
            let zoom = if let Some(z) = state.temp_zoom { z } else { state.zoom };
            write!(out, "{} {}X ", cursor::Goto(
                window.x+window.w - 19, 2
            ), zoom).unwrap();
        },

        r_id: (FocusType::Param, 0),
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
        r: |out, window, id, state, focus| {
            let tempo = if let Some(t) = state.temp_tempo { t } else { state.tempo };
            tempo::render(out, window.x+window.w-3, window.y, tempo, state.tick);
        },

        g_id: (FocusType::Param, 0),
        g_t: |a, id, state| match a {
            Action::Up => Action::SetMeter(state.meter_beat + 1, state.meter_note),
            Action::Down => Action::SetMeter(state.meter_beat - 1, state.meter_note),
            _ => Action::Noop,
        },
        g: |out, window, id, state, focus|
            write!(out, "{} {} ", cursor::Goto(
                window.x+window.w-14, 2
            ), state.meter_beat).unwrap(),
        
        p_id: (FocusType::Param, 0),
        p_t: |a, id, state| match a {
            Action::Up => Action::SetMeter(state.meter_beat, state.meter_note + 1),
            Action::Down => Action::SetMeter(state.meter_beat, state.meter_note - 1),
            _ => Action::Noop,
        },
        p: |out, window, id, state, focus|
            write!(out, "{} {} ", cursor::Goto(
                window.x+window.w-14, 3
            ), state.meter_note).unwrap(),

        active: None,
    }
}