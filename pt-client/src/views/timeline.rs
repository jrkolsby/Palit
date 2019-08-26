use std::fs::File;

use cursive::view::{View, ViewWrapper};
use cursive::views::{DummyView, LinearLayout, Button, Dialog};
use cursive::event::{Event, EventResult};
use cursive::theme::{Color, BaseColor};

use cursive::wrap_impl;

use crate::components::{Splash, SplashAsset, Waveform, alert};

use crate::utils::{TimelineState};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

pub struct Timeline<T: View> {
    state: TimelineState,
    layout: T
}

pub enum Action {
    Add_Note,
    Arm,
    Edit_Mode,
    Loop_Mode,
    Pitch,
    Volume,
    Select_Y, // Yellow
    Select_G, // Green
    Select_P, // Pink
    Select_B, // Blue
}

fn reducer(state: &TimelineState, action: Action) -> &TimelineState {
    state
}

impl Timeline<LinearLayout> {
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

impl <T: View> ViewWrapper for Timeline<T> {
    wrap_impl!(self.layout: T);
}