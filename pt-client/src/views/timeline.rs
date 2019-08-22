use std::fs::File;

use cursive::Cursive;

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

// state
struct Timeline<> {
    name: String,
    //tracks: Vec<Track>,
    //metronome: Tempo,
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
        let mut default_state = Timeline {
            name: "Verse".to_string(),
            //tracks: Vec::new(),
            //metronome: Tempo::new()
        };

        default_state
    }

    pub fn render(&self, screen: &mut Cursive) {
        // screen.add_layer(Tempo::new())
        // screen.add_layer(Track::new())
    }

    pub fn destroy() {
        // screen.pop_layer()
    }
}
