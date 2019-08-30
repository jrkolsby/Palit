extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor, terminal_size};
use termion::raw::{RawTerminal};

use std::fs::File;
use std::io::{Write, Stdout, BufReader};

use crate::components::{waveform};
use crate::common::{Action, Asset, Track, Region, file_to_pairs};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

const MARGIN: (u16, u16) = (3, 3);

pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    project: File,
    pairs_example: Vec<(i32, i32)>,
    state: TimelineState,
}

#[derive(Clone, Debug)]
pub struct TimelineState {
    pub name: String,
    pub tempo: u16,
    pub time_beat: usize, // TOP 
    pub time_frac: usize, // BOTTOM
    pub sequence: Vec<Track>, // TRACKS
    pub assets: Vec<Asset> // FILES
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    state.clone()
}

impl Timeline {
    pub fn new() -> Self {

        // Initialize State
        let initial_state: TimelineState = TimelineState {
            name: "Wowee".to_string(),
            tempo: 127,
            time_beat: 4, // TOP 
            time_frac: 4, // BOTTOM
            sequence: vec![
                Track {
                    id: 0,
                    regions: vec![
                        Region {
                            id: 0,
                            asset_id: 0,
                            asset_in: 0,
                            asset_out: 448000,
                            offset: 0,

                        }
                    ]
                }
            ], // TRACKS
            assets: vec![
                Asset {
                    id: 0,
                    src: "test.wav".to_string(),
                    sample_rate: 44800,
                    duration: 448000,
                    channels: 2
                }

            ] // FILES
        };

        // Load logo asset
        let project_file = File::open("storage/project.xml").unwrap();

        let asset_file = WaveFile::open("storage/test.wav").unwrap();
        let pairs: Vec<(i32, i32)> = file_to_pairs(asset_file, 10, 4);

        // Calculate center position
        let size: (u16, u16) = terminal_size().unwrap();

        Timeline {
            x: MARGIN.0,
            y: MARGIN.1,
            pairs_example: pairs,
            width: size.0 - (MARGIN.0*2),
            height: size.1 - (MARGIN.1*2),
            state: initial_state,
            project: project_file,
        }
    }

    pub fn render(&self, mut out: RawTerminal<Stdout>) -> RawTerminal<Stdout> {

        write!(out, "{}{}{} {} {} ",
            cursor::Goto(self.x,self.y),
            color::Bg(color::Magenta),
            color::Fg(color::Black),
            "Hello".to_string(),
            self.state.name).unwrap();

        out = waveform::render(out, &self.pairs_example, self.x + 12, self.y);

        write!(out, "{}", color::Bg(color::Reset)).unwrap();
        out.flush().unwrap();

        out
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = reduce(self.state.clone(), action);
    }
}