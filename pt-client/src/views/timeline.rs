use std::fs::File;

use cursive::{Printer};
use cursive::views::{DummyView, LinearLayout, Button, Dialog};
use cursive::event::{Event, EventResult};
use cursive::theme::{Color, BaseColor};
use cursive::direction::Direction;
use cursive::vec::Vec2;

use crate::components::{Splash, SplashAsset, Waveform, alert};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

// state
pub struct Timeline {
    state: TimelineState,
    pub layout: LinearLayout
}

// props
pub struct TimelineState {
    pub origin_x: i32,
    pub origin_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub name: String,
    //xml_file: File,
}

impl Timeline {
    pub fn new(default_state: TimelineState) -> Self {
        Timeline {
            state: default_state,
            layout: LinearLayout::vertical()
                .child(Splash::new(SplashAsset::Keyboard, "C#m"))
                .child(DummyView)
                .child(Waveform::new(Color::Light(BaseColor::Magenta)))
                .child(Button::new("Save and quit", |s| {
                    s.pop_layer();
                }))
        }
    }
}