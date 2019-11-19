mod pcm;
mod action;
mod color;
mod render;
mod multifocus;

pub use pcm::file_to_pairs;
pub use pcm::generate_waveforms;
pub use pcm::beat_offset;
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
pub use multifocus::FocusType;
pub use multifocus::ID;

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
pub type DocID = u16;

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

pub trait Module {
    fn read_state(&self, object: xmltree::Element);
    fn write_state(&self) -> xmltree::Element;
}

pub type DirectAction = (u16, Action);

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}