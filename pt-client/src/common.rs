use termion::raw::{RawTerminal};
use std::io::{BufWriter};
use std::fs::{File};

mod pcm;
mod color;
mod multifocus;
mod layout;
mod files;

pub use pcm::generate_waveform;
pub use pcm::generate_waveforms;
pub use pcm::generate_partial_waveform;
pub use pcm::char_offset;
pub use pcm::offset_char;
pub use pcm::Asset;
pub use pcm::AudioRegion;
pub use pcm::MidiRegion;
pub use pcm::Track;
pub use pcm::REGIONS_PER_TRACK;

pub use color::Color;
pub use color::write_bg;
pub use color::write_fg;

pub use multifocus::Focus;
pub use multifocus::MultiFocus;
pub use multifocus::shift_focus;
pub use multifocus::render_focii;
pub use multifocus::focus_dispatch;
pub use multifocus::FocusType;
pub use multifocus::ID;
pub use multifocus::VOID_ID;

pub use layout::TRACKS_X;
pub use layout::REGIONS_X;
pub use layout::TIMELINE_Y;
pub use layout::MARGIN_D0;
pub use layout::MARGIN_D1;
pub use layout::MARGIN_D2;

pub use files::get_files;
pub use files::PALIT_PROJECTS;
pub use files::PALIT_MODULES;

pub type Screen = RawTerminal<BufWriter<File>>;

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NW,
    NE,
    SW,
    SE,
}

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}
