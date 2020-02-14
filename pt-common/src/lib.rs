extern crate xmltree;

pub mod action;
pub use action::Action;

pub mod pcm;
pub use pcm::Note;
pub use pcm::Route;
pub use pcm::Anchor;
pub use pcm::Module;
pub use pcm::Volume;
pub use pcm::Offset;
pub use pcm::Key;
pub use pcm::Param;

pub mod document;
pub use document::mark_map;
pub use document::param_map;
pub use document::read_document;
pub use document::mark_add;
pub use document::param_add;
pub use document::Document;
pub use document::note_list;