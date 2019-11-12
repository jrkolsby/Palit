mod timeline;
mod action;
mod color;
mod render;
mod multifocus;

pub use timeline::Asset;
pub use timeline::Region;
pub use timeline::Track;
pub use timeline::TimelineState;
pub use timeline::beat_offset;
pub use timeline::file_to_pairs;
pub use timeline::read_document;

pub use action::Action;

pub use color::Color;
pub use color::write_bg;
pub use color::write_fg;

pub use multifocus::Focus;
pub use multifocus::MultiFocus;
pub use multifocus::shift_focus;
pub use multifocus::render_focii;

#[derive(Clone, Debug)]
pub enum Rate {
    Fast,
    Med,
    Slow,
}

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

pub trait Module<T> {
    fn read_state(self, object: xmltree::Element) -> T;
}

pub type DirectAction = (u16, Action);