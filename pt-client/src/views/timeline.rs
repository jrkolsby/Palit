extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor};
use termion::raw::{RawTerminal};

use xmltree::Element;

use std::fs::File;
use std::io::{Write, Stdout, BufReader};
use std::collections::HashMap;

use crate::components::{waveform, tempo, button, ruler};
use crate::common::{Action, Asset, Track, Region, file_to_pairs, Color};
use crate::common::{read_document};
use crate::views::{Layer};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

const MARGIN: (u16, u16) = (3, 3);
const EXTRAS_W: u16 = 6;
const EXTRAS_H: u16 = 5;
const ASSET_PREFIX: &str = "storage/";

// STATIC PROPERTIES THROUGHOUT VIEW'S LIFETIME
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    project: Element,
    waveforms: HashMap<u32, Vec<(i32, i32)>>,
    state: TimelineState,
}

// DYNAMIC PROPERTIES
#[derive(Clone, Debug)]
pub struct TimelineState {
    pub name: String,
    pub tempo: u16,             // TEMPO
    pub time_beat: usize,       // TOP 
    pub time_note: usize,       // BOTTOM
    pub duration_measure: usize,
    pub duration_beat: usize,
    pub zoom: i32,              // BEATS per tick
    pub loop_mode: bool,        // TRUE FOR LOOP
    pub sequence: Vec<Track>,   // TRACKS
    pub assets: Vec<Asset>,      // FILES

    pub tick: bool,

    pub scroll_x: u16,
    pub scroll_y: u16,
    pub focus: usize, 
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    TimelineState {
        name: state.name.clone(),
        tempo: state.tempo.clone(),
        time_beat: match action {
            Action::Up => (state.time_beat + 1),
            Action::Down => (if state.time_beat == 1 { 1 } else { state.time_beat - 1 }),
            _ => state.time_beat,
        },
        time_note: state.time_note.clone(),
        zoom: state.zoom.clone(),
        assets: state.assets.clone(),
        sequence: state.sequence.clone(),
        loop_mode: state.loop_mode.clone(),
        focus: state.focus.clone(),
        scroll_x: match action {
            Action::Right => (state.scroll_x + 1),
            Action::Left => (if state.scroll_x == 0 { 0 } else { state.scroll_x - 1 }),
            _ => state.scroll_x.clone(),
        },
        scroll_y: state.scroll_y.clone(), 
        tick: match action {
            Action::SelectR => !state.tick,
            _ => state.tick.clone(),
        },
        duration_beat: state.duration_beat.clone(),
        duration_measure: state.duration_measure.clone(),
    }
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, project: String) -> Self {

        let xml: Element = read_document(project);

        // Initialize State
        let initial_state: TimelineState = TimelineState {
            name: "Wowee".to_string(),
            tempo: 127,
            time_beat: 4, // TOP 
            time_note: 4, // BOTTOM
            duration_beat: 0,
            duration_measure: 15,
            zoom: 0,
            loop_mode: false,
            sequence: vec![],
            assets: vec![],
            focus: 0,
            scroll_x: 0,
            scroll_y: 0,
            tick: false,
        };

        let mut waveforms: HashMap<u32, Vec<(i32, i32)>> = HashMap::new();

        for asset in initial_state.assets.iter() {
            let asset_src = format!("{}{}", ASSET_PREFIX, asset.src);
            eprintln!("DRAWING {}", asset_src);
            let asset_file = WaveFile::open(asset_src).unwrap();
            let pairs: Vec<(i32, i32)> = file_to_pairs(asset_file, width as usize, 4);
            waveforms.insert(asset.id, pairs);

            /* HASHMAP FNS 
            insert(u32, Vec)
            get(u32)
            remove(u32)
            */
        }

        Timeline {
            x: x,
            y: y,
            width: height,
            height: width,
            waveforms: waveforms,
            state: initial_state,
            project: xml,
        }
    }
}

impl Layer for Timeline {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        // PRINT TEMPO
        out = tempo::render(out, self.x + self.width-3, self.y,
            self.state.time_beat,
            self.state.time_note,
            self.state.duration_measure,
            self.state.duration_beat,
            self.state.tempo,
            self.state.tick,
            self.state.focus == 0,
        ); // top right corner

        out = button::render(out, 2, self.height-3, 56, 
            "RECORD", Color::Red, true);

        out = button::render(out, 60, self.height-3, 19, 
            "IMPORT", Color::Pink, true);

        // PRINT TEMPO
        out = ruler::render(out, 5, 6, self.width-4,
            self.state.time_beat,
            self.state.zoom,
            self.state.scroll_x);
            
        // SAVE AND QUIT
        write!(out, "{}{}{}  Save and quit  {}{}",
            cursor::Goto(self.x+2, self.y+1),
            color::Bg(color::Yellow),
            color::Fg(color::Black),
            color::Bg(color::Reset),
            color::Fg(color::White)).unwrap();

        // PRINT TRACKS
        for (i, track) in self.state.sequence.iter().enumerate() {
            let track_y: u16 = self.y + EXTRAS_H + 2 + (i as u16)*2;

            // PRINT TRACK NUMBER
            write!(out, "{}{}",
                cursor::Goto(self.x, track_y),
                i+1).unwrap();

            // PRINT REGIONS
            for region in track.regions.iter() {
                let id: u32 = region.asset_id;
                let pairs: Vec<(i32, i32)> = self.waveforms.get(&id).unwrap().to_vec();
                out = waveform::render(out, &pairs, self.x+EXTRAS_W, track_y);
            }
        }

        // Render Title
        for i in 1..self.width+1 {
            write!(out, "{}─", cursor::Goto(i,self.y)).unwrap();
        }
        let name_len: u16 = self.state.name.len() as u16;
        let name_x: u16 = self.x + (self.width/2) - (name_len/2);
        write!(out, "{} {} ",
            cursor::Goto(name_x,self.y),
            self.state.name).unwrap();

        write!(out, "{}", color::Bg(color::Reset)).unwrap();

        out.flush().unwrap();

        out
    }
    fn dispatch(&mut self, action: Action) -> Action {
        self.state = reduce(self.state.clone(), action);
        Action::Noop
    }
    fn undo(&mut self) {
        self.state = self.state.clone()
    }
    fn redo(&mut self) {
        self.state = self.state.clone()
    }
    fn alpha(&self) -> bool {
        false
    }
}