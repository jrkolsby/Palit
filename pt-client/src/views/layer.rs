use xmltree::Element;
use libcommon::Action;
use crate::common::Screen;

pub trait Layer {
    fn render(&self, out: &mut Screen, target: bool);
    fn dispatch(&mut self, a: Action) -> Action;
    fn alpha(&self) -> bool;
    fn save(&self) -> Option<Element>;
}