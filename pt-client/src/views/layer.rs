use libcommon::Action;

use crate::common::{Screen};

pub trait Layer {
    fn render(&self, out: &mut Screen, target: bool);
    fn dispatch(&mut self, a: Action) -> Action;
    fn undo(&mut self);
    fn redo(&mut self);
    fn alpha(&self) -> bool;
}