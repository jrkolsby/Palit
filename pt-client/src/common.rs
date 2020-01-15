use termion::raw::{RawTerminal};
use std::io::{BufWriter};
use std::fs::{File};

mod pcm;
mod action;
mod color;
mod multifocus;
mod layout;
mod files;

pub use pcm::file_to_pairs;
pub use pcm::generate_waveforms;
pub use pcm::char_offset;
pub use pcm::offset_char;
pub use pcm::Asset;
pub use pcm::Region;
pub use pcm::Track;

pub use action::Action;

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
pub enum Rate {
    Fast,
    Med,
    Slow,
}

// All parameters range from -1000 to +1000
pub type Param = i16;
pub type Offset = u32;
pub type Key = u8;

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

#[derive(Clone, Debug)]
pub struct Note {
    pub id: u16,
    pub note: Key,
    pub vel: f32,
    pub t_in: Offset,
    pub t_out: Offset,
}

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[derive(Clone, Debug)]
pub struct Route {
    pub id: u16,
    pub patch: Vec<Anchor>,
}

#[derive(Clone, Debug)]
pub struct Anchor {
    pub index: u16,
    pub module_id: u16,
    pub name: String,
    pub input: bool,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub id: u16,
    pub name: String,
}