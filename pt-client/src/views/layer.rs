use std::io::{Stdout};

use termion::raw::{RawTerminal};

use crate::common::{Action};

pub trait Layer {
    fn render(&self, out: RawTerminal<Stdout>) -> RawTerminal<Stdout>;
    fn dispatch(&mut self, a: Action) -> Action;
    fn undo(&mut self);
    fn redo(&mut self);
    fn alpha(&self) -> bool;
}