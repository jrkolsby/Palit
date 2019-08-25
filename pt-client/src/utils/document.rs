extern crate quick-xml;

use crate::views::TimelineState;

struct Asset {
    id: u32,
    name: String,
    src: File,
    sample_rate: u32,
    duration: u32,
    channels: usize
}

struct Clip {
    id: u32,
    asset_id: u32,
    asset_in: i32,
    asset_out: i32,
    name: String,
    offset: i32,
}

struct Track {
    clips: Vec<Clip>
}

fn file_build() {
    TimelineState {
        tempo: u16,
        time_beat: usize, // TOP 
        time_frac: usize, // BOTTOM
        sequence: Vec<Track>, // TRACKS
        assets: Vec<Asset> // 
    }
}

fn read_document(in: File) -> TimelineState {

}