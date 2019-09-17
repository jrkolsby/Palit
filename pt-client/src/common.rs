mod timeline;
mod multifocus;
mod action;
mod color;
mod render;

pub use timeline::Asset;
pub use timeline::Region;
pub use timeline::Track;
pub use timeline::beat_offset;
pub use timeline::file_to_pairs;
pub use timeline::read_document;

pub use action::Action;

pub use multifocus::MultiFocus;

pub use color::Color;

#[derive(Clone, Debug)]
pub enum Rate {
    Fast,
    Med,
    Slow,
}