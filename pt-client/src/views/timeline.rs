extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor};
use termion::raw::{RawTerminal};

use xmltree::Element;

use std::fs::File;
use std::io::{Write, Stdout, BufReader};
use std::collections::HashMap;

use crate::components::{waveform, tempo, button, ruler};
use crate::common::{Action, Asset, Track, Region, Color, Rate};
use crate::common::{read_document, beat_offset, file_to_pairs};
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
    pub zoom: usize,              // BEATS per tick
    pub loop_mode: bool,        // TRUE FOR LOOP
    pub sequence: Vec<Track>,   // TRACKS
    pub assets: Vec<Asset>,      // FILES
    pub sample_rate: Rate,

    pub tick: bool,

    pub scroll_x: u16,
    pub scroll_y: u16,
    pub focus: usize, 

    pub playhead: u16,
}

fn reduce(state: TimelineState, action: Action) -> TimelineState {
    let playhead = match action {
        Action::Tick => state.playhead + 1,
        _ => state.playhead.clone(),
    };
    TimelineState {
        name: state.name.clone(),
        tempo: state.tempo.clone(),
        time_beat: match action {
            Action::Up => (state.time_beat + 1),
            Action::Down => (if state.time_beat == 1 { 1 } else { state.time_beat - 1 }),
            _ => state.time_beat,
        },
        time_note: state.time_note.clone(),
        sample_rate: state.sample_rate.clone(),
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
        tick: (playhead % 2) == 0,
        duration_beat: state.duration_beat.clone(),
        duration_measure: state.duration_measure.clone(),
        playhead,
    }
}

fn generate_waveforms(state: &TimelineState) 
    -> HashMap<u32, Vec<(i32, i32)>> {
    let mut waveforms: HashMap<u32, Vec<(i32, i32)>> = HashMap::new();

    for asset in state.assets.iter() {
        eprintln!("generating wave: {}", asset.src.clone());

        let asset_file = WaveFile::open(asset.src.clone()).unwrap();

        let num_pairs: usize = beat_offset(
            asset.duration,
            state.sample_rate.clone(),
            state.tempo,
            state.zoom) as usize;

        let pairs: Vec<(i32, i32)> = file_to_pairs(asset_file, num_pairs, 4);

        waveforms.insert(asset.id, pairs);
    }
    waveforms
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, project_src: String) -> Self {

        let project: Element = read_document(project_src);

        // Initialize State
        let initial_state: TimelineState = TimelineState {
            name: "Wowee".to_string(),
            tempo: 127,
            time_beat: 4, // TOP 
            time_note: 4, // BOTTOM
            duration_beat: 0,
            duration_measure: 15,
            zoom: 1,
            loop_mode: false,
            sequence: vec![
                Track {
                    id: 0,
                    color: Color::Yellow,
                    regions: vec![
                        Region {
                            id: 0,
                            asset_id: 0,
                            offset: 448000,
                            asset_in: 0,
                            asset_out: 448000,
                        }
                    ]
                }
            ],
            assets: vec![
                Asset {
                    id: 0,
                    src: "/Users/jrkolsby/Work/Palit/storage/assets/Keyboard.wav".to_string(),
                    sample_rate: 48000,
                    duration: 448000,
                    channels: 2
                }

            ],
            focus: 0,
            scroll_x: 0,
            scroll_y: 0,
            tick: true,
            playhead: 0,
            sample_rate: Rate::Fast,
        };

        Timeline {
            x,
            y,
            width,
            height,
            project,
            waveforms: generate_waveforms(&initial_state),
            state: initial_state,
/*
 *                O        O
 *                      \_______
 *      Timeline         __---"
 *      wizard
 */
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
        out = ruler::render(out, 5, 6, 
            self.width-4,
            self.height,
            self.state.time_beat,
            self.state.zoom,
            self.state.scroll_x,
            self.state.playhead);
            
        // SAVE AND QUIT
        write!(out, "{}{}{}  Save and quit  {}{}",
            cursor::Goto(self.x+2, self.y+1),
            color::Bg(color::Yellow),
            color::Fg(color::Black),
            color::Bg(color::Reset),
            color::Fg(color::White)).unwrap();

        // Print track sidebar
        for (i, track) in self.state.sequence.iter().enumerate() {
            let track_y: u16 = self.y + EXTRAS_H + 2 + (i as u16)*2;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(self.x, track_y),
                i+1).unwrap();
        }

        // Print regions
        for i in 1..self.width+1 {
            write!(out, "{}─", cursor::Goto(i,self.y)).unwrap();

            for (j, track) in self.state.sequence.iter().enumerate() {
                let track_y: u16 = self.y + 10 + (j as u16 * 2);
                eprintln!("track_y {}", track_y);

                // PRINT REGIONS
                for region in track.regions.iter() {
                    let id: u32 = region.asset_id;
                    let offset: u32 = region.offset;

//pub fn beat_offset(delay: u32, sample_rate: u32, bpm: u16, zoom: usize) -> u32 {
                    if beat_offset(offset, 
                        self.state.sample_rate.clone(),
                        self.state.tempo,
                        self.state.zoom) == (i + self.state.scroll_x).into() {
                            out = waveform::render(out, &self.waveforms[&id], self.x+i, track_y);
                        }
                }
            }
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