use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, MultiFocus, FocusType, ID, Window};
use crate::common::{REGIONS_X, TIMELINE_Y};
use crate::common::{offset_char, char_offset};
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

        r_id: (FocusType::Button, 0),
        r_t: |a, id, state| match a { 
            Action::SelectR => Action::Record,
            // Will be dispatched immediately after record is pressed
            a @ Action::AddMidiRegion(_,_,_,_) |
            a @ Action::AddNote(_) => a,
            _ => Action::Noop 
        },
        r: |out, window, id, state, focus| 
            write!(out, "{} RECORD ", cursor::Goto(window.x + 2, window.y)).unwrap(),

        y_id: (FocusType::Button, 0),
        y_t: |a, id, state| match a { 
            Action::Up |
            Action::Down |
            Action::SelectY => Action::LoopMode(!state.loop_mode),
            _ => Action::Noop 
        },
        y: |out, window, id, state, focus| 
            write!(out, "{} {} ", cursor::Goto(window.x + 2, window.y + 2), if state.loop_mode { 
                "LOOP ON" 
            } else { 
                "LOOP OFF"
            }).unwrap(),

        g_id: (FocusType::Button, 0),
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

        p_id: (FocusType::Button, 1),
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

        b_id: void_id.clone(),
        b_t: void_transform,
        b: void_render,

        active: None,
    }
}