use std::convert::TryInto;
use std::collections::HashMap;
use libcommon::{Param, param_map, mark_map, mark_add, param_add, note_list};

use xmltree::Element;

use crate::views::{Timeline, TimelineState};
use crate::common::{AudioRegion, MidiRegion, Track, Asset, REGIONS_PER_TRACK};

pub fn write(state: TimelineState) -> Element {
    let mut root = Element::new("timeline");

    param_add(&mut root, state.tempo, "bpm".to_string());
    param_add(&mut root, state.meter_beat, "meter_beat".to_string());
    param_add(&mut root, state.meter_note, "meter_note".to_string());
    mark_add(&mut root, state.seq_in, "seq_in".to_string());
    mark_add(&mut root, state.seq_out, "seq_out".to_string());
    mark_add(&mut root, state.loop_in, "loop_in".to_string());
    mark_add(&mut root, state.loop_out, "loop_out".to_string());

    for (id, asset) in state.assets.iter() {
        let mut asset_el = Element::new("asset");
        asset_el.attributes.insert("id".to_string(), id.to_string());
        asset_el.attributes.insert("size".to_string(), asset.duration.to_string());
        asset_el.attributes.insert("src".to_string(), asset.src.to_string());
        root.children.push(asset_el);
    }

    for (t_id, track) in state.tracks.iter() {
        let mut track_el = Element::new("track");
        track_el.attributes.insert("id".to_string(), t_id.to_string());

        for (r_id, audio_region) in state.regions.iter() {
            let track_id = r_id / REGIONS_PER_TRACK;
            let local_id = r_id % REGIONS_PER_TRACK;

            if audio_region.track == track_id {
                let mut audio_el = Element::new("audio");
                audio_el.attributes.insert("id".to_string(), local_id.to_string());
                audio_el.attributes.insert("asset".to_string(), audio_region.asset_id.to_string());
                audio_el.attributes.insert("in".to_string(), audio_region.asset_in.to_string());
                audio_el.attributes.insert("duration".to_string(), audio_region.duration.to_string());
                audio_el.attributes.insert("offset".to_string(), audio_region.offset.to_string());
                track_el.children.push(audio_el);
            }
        }

        for (r_id, midi_region) in state.midi_regions.iter() {
            let track_id = r_id / REGIONS_PER_TRACK;
            let local_id = r_id % REGIONS_PER_TRACK;

            if midi_region.track == track_id {
                let mut midi_el = Element::new("midi");
                midi_el.attributes.insert("id".to_string(), local_id.to_string());
                midi_el.attributes.insert("offset".to_string(), midi_region.offset.to_string());
                midi_el.attributes.insert("duration".to_string(), midi_region.duration.to_string());
                for note in midi_region.notes.iter() {
                    let mut note_el = Element::new("note");
                    note_el.attributes.insert("id".to_string(), note.id.to_string());
                    note_el.attributes.insert("key".to_string(), note.note.to_string());
                    note_el.attributes.insert("vel".to_string(), note.vel.to_string());
                    note_el.attributes.insert("t_in".to_string(), note.t_in.to_string());
                    note_el.attributes.insert("t_out".to_string(), note.t_out.to_string());
                    midi_el.children.push(note_el);
                }
                track_el.children.push(midi_el);
            }
        }
        root.children.push(track_el);
    }

    root
}

pub fn read(mut doc: Element) -> TimelineState {

    let (mut doc, params) = param_map(&mut doc);
    let (mut doc, marks) = mark_map(&mut doc);

    let mut state = TimelineState {

        tempo: *params.get("bpm").unwrap_or(&127.0) as u16,
        meter_beat: *params.get("meter_beat").unwrap_or(&4.0) as u16,
        meter_note: *params.get("meter_note").unwrap_or(&4.0) as u16,
        seq_in: *marks.get("seq_in").unwrap_or(&0),
        seq_out: *marks.get("seq_out").unwrap_or(&48000),
        loop_in: *marks.get("loop_in").unwrap_or(&0),
        loop_out: *marks.get("loop_out").unwrap_or(&0),
        sample_rate: 48_000,
        tracks: HashMap::new(),
        assets: HashMap::new(),
        regions: HashMap::new(),
        midi_regions: HashMap::new(),

        loop_mode: false,
        tick: true,
        playhead: 0,
        zoom: 1,
        scroll_x: 0,
        scroll_y: 0,
        scroll_mid: 0,
        temp_tempo: None,
        temp_zoom: None,
        focus: (0,0),
    };

    // keep track of track index for vertical positioning
    let mut counter: u16 = 0;

    while let Some(mut track) = doc.take_child("track") {
        let t_id: &str = track.attributes.get("id").unwrap();
        let _t_id = t_id.parse::<u16>().unwrap();

        state.tracks.insert(_t_id, Track {
            id: _t_id,
            record: 0,
            mute: false,
            solo: false,
            monitor: true,
            index: counter,
        });

        while let Some(audio_region) = track.take_child("audio") {

            let r_id: &str = audio_region.attributes.get("id").unwrap();
            let a_id: &str = audio_region.attributes.get("asset").unwrap();
            let offset: &str = audio_region.attributes.get("offset").unwrap();
            let a_in: &str = audio_region.attributes.get("in").unwrap();
            let duration: &str = audio_region.attributes.get("duration").unwrap();

            let _r_id = r_id.parse::<u16>().unwrap();            
            let global_r_id = _t_id * REGIONS_PER_TRACK + _r_id;

            state.regions.insert(global_r_id, AudioRegion {
                asset_id: a_id.parse().unwrap(),
                asset_in: a_in.parse().unwrap(),
                duration: duration.parse().unwrap(),
                offset: offset.parse().unwrap(),
                track: _t_id,
            });
        }

        while let Some(mut midi_region) = track.take_child("midi") {
            let r_id: &str = midi_region.attributes.get("id").unwrap();

            let _r_id = r_id.parse::<u16>().unwrap();
            let global_r_id = _t_id * REGIONS_PER_TRACK + _r_id;

            let (midi_region, notes) = note_list(&mut midi_region, _r_id);

            let duration = midi_region.attributes.get("duration").unwrap();
            let offset = midi_region.attributes.get("offset").unwrap();

            state.midi_regions.insert(global_r_id, MidiRegion {
                duration: duration.parse().unwrap(),
                offset: offset.parse().unwrap(),
                notes,
                track: _t_id,
            });
        }

        counter += 1;
    }

    while let Some(asset) = doc.take_child("asset") {
        let a_id: &str = asset.attributes.get("id").unwrap();
        let duration: &str = asset.attributes.get("size").unwrap();
        state.assets.insert(a_id.parse::<u16>().unwrap(), Asset {
            src: asset.attributes.get("src").unwrap().parse().unwrap(),
            duration: duration.parse().unwrap(),
            channels: 2,
            waveform: vec![],
        });
    }
    
    eprintln!("READ {:?}", state);

    return state;
}