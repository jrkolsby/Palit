// A quickly made Hammond organ.

extern crate alsa;
extern crate sample;
extern crate wavefile;

use std::{iter, error};
use sample::signal;

use wavefile::{WaveFile, WaveFileIterator};

mod core;
mod synth;

use crate::core::{SF, SigGen, write_samples_io, write_samples_direct, open_audio_dev};
use crate::synth::{Synth};

fn main() -> Result<(), Box<error::Error>> {
    let (audio_dev, rate) = open_audio_dev()?;

    let wav: WaveFile = WaveFile::open("Who.wav").unwrap();
    let wav_iter: WaveFileIterator = wav.iter();

    // 256 Voices synth
    let mut synth = Synth {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(rate)),
        stored_sample: None,
        bar_values: [1., 0.75, 1., 0.75, 0., 0., 0., 0., 0.75], // Some Gospel-ish default.
	wave_file: wav_iter,
    };

    // Create an array of fds to poll.
    use alsa::PollDescriptors;
    let mut fds = audio_dev.get()?;
    
    // Let's use the fancy new "direct mode" for minimum overhead!
    let mut mmap = audio_dev.direct_mmap_playback::<SF>();
    
    // Direct mode unavailable, use alsa-lib's mmap emulation instead
    let mut io = if mmap.is_err() {
        Some(audio_dev.io_i16()?)
    } else { None };

    // Play minor 7
    synth.add_note(86, 0.5);
    synth.add_note(89, 0.5);
    synth.add_note(92, 0.5);

    loop {
        if let Ok(ref mut mmap) = mmap {
            if write_samples_direct(&audio_dev, mmap, &mut synth)? { continue; }
        } else if let Some(ref mut io) = io {
            if write_samples_io(&audio_dev, io, &mut synth)? { continue; }
        }
        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}
