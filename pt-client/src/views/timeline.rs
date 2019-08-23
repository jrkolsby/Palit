use std::fs::File;

use cursive::Cursive;
use cursive::views::{Dialog, SelectView, BoxView, DummyView, LinearLayout, EditView, Button};
use cursive::theme::{Color, BaseColor};

use crate::components::{Splash, SplashAsset, Waveform, alert};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

// state
pub struct Timeline<> {
    name: String,
    props: TimelineProps,
    //tracks: Vec<Track>,
    //metronome: Tempo,
}

// props
pub struct TimelineProps {
    pub origin_x: i32,
    pub origin_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    //xml_file: File,
}

impl Timeline {
    pub fn new(props: TimelineProps) -> cursive::views::BoxView<LinearLayout> {
        BoxView::with_full_screen(
            LinearLayout::vertical()
                .child(Splash::new(SplashAsset::Keyboard, "C#m"))
                .child(DummyView)
                .child(DummyView)
                .child(Waveform::new(Color::Light(BaseColor::Magenta)))
                .child(Button::new("Save and quit", |s| {
                    s.pop_layer();
                }))
            
        )
    }

    pub fn destroy() {
        // screen.pop_layer()
    }
}