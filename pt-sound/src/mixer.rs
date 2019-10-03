use sample::{signal, Signal, Sample};
use std::fs::File;
use std::io::Write;

use crate::core::{SF, SigGen};
use crate::synth::{Synth};
use crate::timeline::{Timeline};

pub struct Mixer<'a> {
    pub timeline: Timeline<'a>,
    pub synths: Vec<Synth>
}

impl Iterator for Mixer<'_> { 
    type Item = SF;
    fn next(&mut self) -> Option<Self::Item> {
        let mut z: f64 = 0.0;
        z = z + self.timeline.next().unwrap() as f64;
        for mut s in self.synths.iter_mut() {
            z = z + s.next().unwrap() as f64;
        }
        let z = z.min(0.999).max(-0.999);
        let z: Option<SF> = Some(SF::from_sample(z));
        z
    }
}

