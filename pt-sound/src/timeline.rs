use sample::signal;

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen};

pub struct Timeline<'a> {
    pub wave_file: WaveFileIterator<'a>,
}

impl Iterator for Timeline<'_> { 
    type Item = SF;
    fn next(&mut self) -> Option<Self::Item> {
	let z: f64 = self.wave_file.next().unwrap()[0] as f64;
	let z = z / 20000000.0;
        let z = z.min(0.999).max(-0.999);
        let z: Option<SF> = Some(SF::from_sample(z));
        self.stored_sample = z;
        z
    }
}

