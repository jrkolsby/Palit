use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use cursive::{Cursive, Printer};

use crate::components::track::Track;
use crate::components::tempo::Tempo;

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

// state
struct Timeline<> {
    name: String,
    tracks: Vec<Track>,
    metronome: Tempo,
}

// props
struct TimelineProps {
    origin_x: i32,
    origin_y: i32,
    size_x: i32,
    size_y: i32,
    xml_file: File,
}

impl Timeline {
    pub fn new(props: TimelineProps) -> Self {
        let mut defaultState = Timeline {
            name: "Verse".to_string(),
            tracks: Vec::new(),
            metronome: Tempo::new()
        };

        defaultState
    }

    pub fn render(&self, screen: &mut Cursive) {
        // screen.add_layer(Tempo::new())
        // screen.add_layer(Track::new())
    }

    pub fn destroy() {
        // screen.pop_layer()
    }
}
