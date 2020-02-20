use std::io::Write;
use termion::cursor;
use libcommon::Action;

use crate::common::{Screen, MultiFocus, FocusType, ID, Window};
use crate::common::{TRACKS_X, TIMELINE_Y};
use crate::views::TimelineState;

pub fn new(track_id: u16) -> MultiFocus::<TimelineState> {

    let void_id: ID = (FocusType::Void, 0);
    let void_render: fn(&mut Screen, Window, ID, &TimelineState, bool) =
        |_, _, _, _, _| {};
    let void_transform: fn(Action, ID, &TimelineState) -> Action = 
        |a, _, _| a;

    MultiFocus::<TimelineState> {
        w_id: (FocusType::Button, track_id),
        w: void_render,

        r_id: void_id.clone(),
        r_t: |action, id, state| match action {
            Action::SelectR => Action::RecordTrack(
                id.1, 
                (state.tracks.get(&id.1).unwrap().record + 1) % 3
            ),
            _ => Action::Noop,
        },
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

        g_id: void_id.clone(),
        g_t: |action, id, state| match action {
            Action::SelectG => Action::MuteTrack(
                id.1,
                !state.tracks.get(&id.1).unwrap().mute
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

        b_id: void_id.clone(),
        b_t: |action, id, state| match action {
            Action::SelectB => Action::SoloTrack(
                id.1,
                !state.tracks.get(&id.1).unwrap().solo
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

        p_id: void_id.clone(),
        p_t: |action, id, state| match action {
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

        y_id: void_id.clone(),
        y_t: void_transform,
        y: void_render,

        active: None,
    }
}