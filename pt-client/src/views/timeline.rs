extern crate wavefile;

use wavefile::WaveFile;

use termion::{color, cursor};
use termion::raw::{RawTerminal};

use std::io::{Write, Stdout};
use std::collections::HashMap;

use crate::components::{waveform, tempo, button, ruler};
use crate::common::{Action, Color, TimelineState};
use crate::common::{read_document, beat_offset, file_to_pairs};
use crate::views::{Layer};

//#[derive(Debug)] TODO: Implement {:?} fmt for Track and Tempo

const MARGIN: (u16, u16) = (3, 3);
const EXTRAS_W: u16 = 6;
const EXTRAS_H: u16 = 7;
const SCROLL_R: u16 = 40;
const SCROLL_L: u16 = 10;
const ASSET_PREFIX: &str = "storage/";

// STATIC PROPERTIES THROUGHOUT VIEW'S LIFETIME
pub struct Timeline {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
    project_src: String,
    waveforms: HashMap<u32, Vec<(i32, i32)>>,
    state: TimelineState,
}


fn reduce(state: TimelineState, action: Action) -> TimelineState {
    let playhead = match action {
        Action::Tick => state.playhead + 1,
        Action::Right => state.playhead + 1,
        Action::Left => if state.playhead == 0 { 0 } else { state.playhead - 1 },
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
        scroll_x: if playhead-state.scroll_x > SCROLL_R { state.scroll_x+1 }
            else if state.scroll_x > 0 && playhead-state.scroll_x < SCROLL_L { state.scroll_x-1 }
            else { state.scroll_x },
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

        eprintln!("num {}", num_pairs);

        let pairs: Vec<(i32, i32)> = file_to_pairs(asset_file, num_pairs, 4);

        waveforms.insert(asset.id, pairs);
    }
    waveforms
}

impl Timeline {
    pub fn new(x: u16, y: u16, width: u16, height: u16, project_src: String) -> Self {

        // Initialize State
        let initial_state: TimelineState = read_document(project_src.clone()); 

        Timeline {
            x,
            y,
            width,
            height,
            project_src,
            waveforms: generate_waveforms(&initial_state),
            state: initial_state,
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
            
        // save and quit
        write!(out, "{}{}{}  Save and quit  {}{}",
            cursor::Goto(self.x+2, self.y+1),
            color::Bg(color::Yellow),
            color::Fg(color::Black),
            color::Bg(color::Reset),
            color::Fg(color::White)).unwrap();

        // Print track sidebar
        for (i, _track) in self.state.sequence.iter().enumerate() {
            let track_y: u16 = self.y + EXTRAS_H + (i*2) as u16;

            // Print track number on left
            write!(out, "{}{}",
                cursor::Goto(self.x+1, track_y),
                i+1).unwrap();
        }

        // Print regions
        for i in 1..self.width+1 {
            write!(out, "{}─", cursor::Goto(i,self.y)).unwrap();

            for (j, track) in self.state.sequence.iter().enumerate() {
		        let track_y: u16 = self.y + EXTRAS_H + (j*2) as u16;

                // PRINT REGIONS
                for region in track.regions.iter() {
                    let id: u32 = region.asset_id;
                    let offset: u32 = region.offset;

                    if beat_offset(offset, 
                        self.state.sample_rate.clone(),
                        self.state.tempo,
                        self.state.zoom) == (i + self.state.scroll_x).into() {
                            out = waveform::render(out, 
                                &self.waveforms[&id], self.x+EXTRAS_W+i, track_y);
                        }
                }
            }
        }
        let name_len: u16 = self.state.name.len() as u16;
        let name_x: u16 = self.x + (self.width/2) - (name_len/2);
        write!(out, "{} {} ",
            cursor::Goto(name_x,self.y),
            self.state.name).unwrap();

        // print tempo
        out = ruler::render(out, 5, 6, 
            self.width-4,
            self.height,
            self.state.time_beat,
            self.state.zoom,
            self.state.scroll_x,
            self.state.playhead);

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
