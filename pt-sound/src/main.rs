// A quickly made Hammond organ.

extern crate alsa;
extern crate sample;

use std::{iter, error};
use alsa::{seq, pcm};
use std::ffi::CString;
use sample::signal;

fn open_audio_dev() -> Result<(alsa::PCM, u32), Box<error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 { 
        println!("Usage: 'cargo run --release CARD_NAME SAMPLE_RATE BUF_SIZE'");
        Err("No card name specified")?
    }
    let req_devname = format!("hw:{}", args[1]);
    let req_samplerate = args.get(2).map(|x| x.parse()).unwrap_or(Ok(48000))?;
    let req_bufsize = args.get(3).map(|x| x.parse()).unwrap_or(Ok(256))?; // A few ms latency by default, that should be nice 
    
    // Open the device
    let p = alsa::PCM::new(&req_devname, alsa::Direction::Playback, false)?;
    
    // Set hardware parameters
    {
        let hwp = pcm::HwParams::any(&p)?;
        hwp.set_channels(2)?;
        hwp.set_rate(req_samplerate, alsa::ValueOr::Nearest)?;
        hwp.set_format(pcm::Format::s16())?;
        hwp.set_access(pcm::Access::MMapInterleaved)?;
        hwp.set_buffer_size(req_bufsize)?;
        hwp.set_period_size(req_bufsize / 4, alsa::ValueOr::Nearest)?;
        p.hw_params(&hwp)?;
    }

    // Set software parameters
    let rate = {
        let hwp = p.hw_params_current()?;
        let swp = p.sw_params_current()?;
        let (bufsize, periodsize) = (hwp.get_buffer_size()?, hwp.get_period_size()?);
        swp.set_start_threshold(bufsize - periodsize)?;
        swp.set_avail_min(periodsize)?;
        p.sw_params(&swp)?;
        println!("Opened audio output {:?} with parameters: {:?}, {:?}", req_devname, hwp, swp);
        hwp.get_rate()?
    };

    Ok((p, rate))
}

// Sample format
type SF = i16;

type SigGen = signal::Sine<signal::ConstHz>;

// Standard Hammond drawbar.
const BAR_FREQS: [f64; 9] = [16., 5.+1./3., 8., 4., 2.+2./3., 2., 1.+3./5., 1.+1./3., 1.];

#[derive(Clone)]
struct Sig {
    note: u8,
    sig: SigGen,
    targetvol: f64,
    curvol: f64,
    baridx: usize,
}

struct Synth {
    sigs: Vec<Option<Sig>>,
    sample_rate: signal::Rate,
    stored_sample: Option<SF>,
    bar_values: [f64; 9],
}

impl Synth {
    fn add_note(&mut self, note: u8, vol: f64) {
        let hz = 440. * 2_f64.powf((note as f64 - 69.)/12.);

        for (baridx, barfreq) in BAR_FREQS.iter().enumerate() {
            let idx = self.sigs.iter().position(|s| s.is_none());
            let idx = if let Some(idx) = idx { idx } else {
                println!("Voice overflow!"); return;
            };
            let hz = self.sample_rate.const_hz(hz * 8. / barfreq);
            let s = Sig { sig: hz.sine(), note, targetvol: vol, curvol: 0., baridx };
            self.sigs[idx] = Some(s);
        }
    }
    fn remove_note(&mut self, note: u8) {
        for i in self.sigs.iter_mut() {
            if let &mut Some(ref mut i) = i {
                if i.note == note { i.targetvol = 0. }
            }
        }
    }
    fn cc(&mut self, ctrl: u32, value: i32) {
        let idx = match ctrl {
            // Standard knobs on UMA25S, modify to your liking
            1 => 0,
            74 => 1,
            71 => 2,
            73 => 3,
            75 => 4,
            72 => 5,
            91 => 6,
            93 => 7,
            10 => 8,
            _ => return,
        };
        self.bar_values[idx] = f64::from(value) / 255.;
    }
}

impl Iterator for Synth { 
    type Item = SF;
    fn next(&mut self) -> Option<Self::Item> {
        use sample::{Signal, Sample};

        // Mono -> Stereo
        if let Some(s) = self.stored_sample.take() { return Some(s) };
        
        let mut z = 0f64;
        for sig in &mut self.sigs { 
            let mut remove = false;
            if let &mut Some(ref mut i) = sig {
                let barvalue = self.bar_values[i.baridx];
                if barvalue > 0.0 {
                    let s = i.sig.next();
                    z += s[0].mul_amp(i.curvol * barvalue);
                }

                // Quick and dirty volume envelope to avoid clicks. 
                if i.curvol != i.targetvol {
                    if i.targetvol == 0. {
                        i.curvol -= 0.002;
                        if i.curvol <= 0. { remove = true; }
                    } else {
                        i.curvol += 0.002;
                        if i.curvol >= i.targetvol { i.curvol = i.targetvol; }
                    }
                }
            }
            if remove { *sig = None };
        }
        let z = z.min(0.999).max(-0.999);
        let z: Option<SF> = Some(SF::from_sample(z));
        self.stored_sample = z;
        z
    }
}

fn write_samples_direct(p: &alsa::PCM, mmap: &mut alsa::direct::pcm::MmapPlayback<SF>, synth: &mut Synth)
    -> Result<bool, Box<error::Error>> {

    if mmap.avail() > 0 {
        // Write samples to DMA area from iterator
        mmap.write(synth);
    }
    use alsa::pcm::State;
    match mmap.status().state() {
        State::Running => { return Ok(false); }, // All fine
        State::Prepared => { println!("Starting audio output stream"); p.start()? },
        State::XRun => { println!("Underrun in audio output stream!"); p.prepare()? },
        State::Suspended => { println!("Resuming audio output stream"); p.resume()? },
        n @ _ => Err(format!("Unexpected pcm state {:?}", n))?,
    }
    Ok(true) // Call us again, please, there might be more data to write
}

fn write_samples_io(p: &alsa::PCM, io: &mut alsa::pcm::IO<SF>, synth: &mut Synth) -> Result<bool, Box<error::Error>> {
    let avail = match p.avail_update() {
        Ok(n) => n,
        Err(e) => {
            println!("Recovering from {}", e);
            if let Some(errno) = e.errno() {
                p.recover(errno as std::os::raw::c_int, true)?;
            }
            p.avail_update()?
        }
    } as usize;

    if avail > 0 {
        io.mmap(avail, |buf| {
            for sample in buf.iter_mut() {
                *sample = synth.next().unwrap()
            };
            buf.len() / 2 
        })?;
    }
    use alsa::pcm::State;
    match p.state() {
        State::Running => Ok(false), // All fine
        State::Prepared => { println!("Starting audio output stream"); p.start()?; Ok(true) },
        State::Suspended | State::XRun => Ok(true), // Recover from this in next round
        n @ _ => Err(format!("Unexpected pcm state {:?}", n))?,
    }
}

fn run() -> Result<(), Box<error::Error>> {
    let (audio_dev, rate) = open_audio_dev()?;

    // 256 Voices synth
    let mut synth = Synth {
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(rate)),
        stored_sample: None,
        bar_values: [1., 0.75, 1., 0.75, 0., 0., 0., 0., 0.75], // Some Gospel-ish default.
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

    synth.add_note(128, 0.5);

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

fn main() {
    println!("Hello, world!");
    if let Err(e) = run() { println!("Error ({}) {}", e.description(), e); }
}
