extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor};
use termion::raw::{RawTerminal};

use std::fs::File;
use std::io::{Write, Stdout, BufReader};
use std::collections::HashMap;

use crate::components::{waveform};
use crate::common::{Action, Asset, Track, Region, file_to_pairs};
use crate::views::{Layer};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

const MARGIN: (u16, u16) = (3, 3);
const EXTRAS_W: u16 = 7;
const EXTRAS_H: u16 = 3;
const ASSET_PREFIX: &str = "storage/";

// STATIC PROPERTIES THROUGHOUT VIEW'S LIFETIME
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    project: File,
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
    pub zoom: f32,              // BEATS per tick
    pub loop_mode: bool,        // TRUE FOR LOOP
    pub sequence: Vec<Track>,   // TRACKS
    pub assets: Vec<Asset>      // FILES
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    TimelineState {
        name: state.name.clone(),
        tempo: state.tempo.clone(),
        time_beat: match action {
            Action::Up => (state.time_beat + 1),
            Action::Down => (state.time_beat - 1),
            _ => state.time_beat,
        },
        time_note: state.time_note.clone(),
        zoom: state.zoom.clone(),
        assets: state.assets.clone(),
        sequence: state.sequence.clone(),
        loop_mode: state.loop_mode.clone(),
    }
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {

        // Initialize State
        let initial_state: TimelineState = TimelineState {
            name: "Wowee".to_string(),
            tempo: 127,
            time_beat: 4, // TOP 
            time_note: 4, // BOTTOM
            zoom: 1.0,
            loop_mode: false,
            sequence: vec![
                Track {
                    id: 0,
                    regions: vec![
                        Region {
                            id: 0,
                            asset_id: 0,
                            asset_in: 0,
                            asset_out: 48000,
                            offset: 0,

                        }
                    ]
                },
                Track {
                    id: 1,
                    regions: vec![
                        Region {
                            id: 0,
                            asset_id: 1,
                            asset_in: 0,
                            asset_out: 48000,
                            offset: 0,
                        }
                    ]
                },
                Track {
                    id: 2,
                    regions: vec![
                        Region {
                            id: 0,
                            asset_id: 2,
                            asset_in: 0,
                            asset_out: 48000,
                            offset: 0,
                        }
                    ]
                }
            ], // TRACKS
            assets: vec![
                Asset {
                    id: 0,
                    src: "Keyboard.wav".to_string(),
                    sample_rate: 44800,
                    duration: 480000,
                    channels: 2
                },
                Asset {
                    id: 1,
                    src: "Loop.wav".to_string(),
                    sample_rate: 44800,
                    duration: 480000,
                    channels: 2
                },
                Asset {
                    id: 2,
                    src: "Who.wav".to_string(),
                    sample_rate: 44800,
                    duration: 480000,
                    channels: 2
                },

            ] // FILES
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

        let project_file = File::open("storage/project.xml").unwrap();

        Timeline {
            x: x,
            y: y,
            width: height,
            height: width,
            waveforms: waveforms,
            state: initial_state,
            project: project_file,
        }
    }
}

impl Layer for Timeline {
    fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        // PRINT NAME
        let name_len: u16 = self.state.name.len() as u16;
        let name_x: u16 = self.x + (self.width/2) - (name_len/2);
        write!(out, "{}{}",
            cursor::Goto(name_x,self.y),
            self.state.name).unwrap();

        let content_x = self.x + EXTRAS_W;

        // PRINT TEMPO
        let mut measure: String = ".".to_string();
        for i in 0..self.state.time_beat-1 {
            measure = format!("{} `", measure);
        }
        let tempo_len: u16 = measure.len() as u16 + 1;
        let n: u16 = self.width / tempo_len;
        for j in 0..n {
            write!(out, "{}{}",
                cursor::Goto(content_x+(j*tempo_len), EXTRAS_H+self.y),
                measure).unwrap()
        }

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