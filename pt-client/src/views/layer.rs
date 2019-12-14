use std::io::{Stdout};

use termion::raw::{RawTerminal};

use crate::common::{Action};

// Every module has an associated view which can render its state
/*
pub enum View {
    Modal,
    Patch,
    Timeline,
    Keyboard,
    Device,
    Hammond,
    Instrument,
    Effect,
    Arpeggio,
}

pub type ID = (View, usize);
*/

pub trait Layer {
    fn render(&self, out: RawTerminal<Stdout>, target: bool) -> RawTerminal<Stdout>;
    fn dispatch(&mut self, a: Action) -> Action;
    fn undo(&mut self);
    fn redo(&mut self);
    fn alpha(&self) -> bool;
    fn shift(&mut self, x: u16, y: u16);
}