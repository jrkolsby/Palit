extern crate sample;
extern crate portaudio;

use std::{iter, error};
use std::ffi::CString;
use std::fs::File;

#[cfg(target_os = "linux")]
extern crate alsa;
#[cfg(target_os = "linux")]
use alsa::{seq, pcm};
#[cfg(target_os = "linux")]
use alsa::pcm::State;
#[cfg(target_os = "linux")]
use alsa::PollDescriptors;

use dsp::{sample::ToFrameSliceMut, NodeIndex, Frame, FromSample, Graph, Node, Sample, Walker};

#[cfg(target_os = "macos")]
use portaudio as pa;

use sample::signal;

use crate::midi::{open_midi_dev, read_midi_event, connect_midi_source_ports};

// SAMPLE FORMAT ALSA
pub type SF = i16;
pub type SigGen = signal::Sine<signal::ConstHz>;

// SAMPLE FORMAT PORTAUDIO
pub type Output = f32;

pub type Phase = f64;
pub type Frequency = f64;
pub type Volume = f32;

const CHANNELS: usize = 2;
const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

#[cfg(target_os = "linux")]
pub fn open_audio_dev() -> Result<(alsa::PCM, u32), Box<error::Error>> {
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

#[cfg(target_os = "linux")]
pub fn write_samples_direct(
    p: &alsa::PCM, 
    mmap: &mut alsa::direct::pcm::MmapPlayback<SF>, 
    synth: &mut Iterator<Item=SF>) -> Result<bool, Box<error::Error>>
{
    if mmap.avail() > 0 {
        mmap.write(&mut Box::new(synth));
    }

    match mmap.status().state() {
        State::Running => { return Ok(false); }, // All fine
        State::Prepared => { println!("Starting audio output stream"); p.start()? },
        State::XRun => { println!("Underrun in audio output stream!"); p.prepare()? },
        State::Suspended => { println!("Resuming audio output stream"); p.resume()? },
        n @ _ => Err(format!("Unexpected pcm state {:?}", n))?,
    }
    Ok(true) // Call us again, please, there might be more data to write
}

#[cfg(target_os = "linux")]
pub fn write_samples_io(
    p: &alsa::PCM, 
    io: &mut alsa::pcm::IO<SF>, 
    synth: &mut Iterator<Item=SF>) -> Result<bool, Box<error::Error>> 
{
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

#[cfg(target_os = "macos")]
pub fn event_loop(ipc_in: File, ipc_client: File, mut patch: Graph<[Output; CHANNELS], DspNode>, master: NodeIndex) -> Result<(), Box<error::Error>> {

    // Set the master node for the graph.
    patch.set_master(Some(master));

    // We'll use this to count down from three seconds and then break from the loop.
    let mut timer: f64 = 10.0;

    // This will be used to determine the delta time between calls to the callback.
    let mut prev_time = None;

    // The callback we'll use to pass to the Stream. It will request audio from our dsp_graph.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, time, .. }| {
        let buffer: &mut [[Output; CHANNELS]] = buffer.to_frame_slice_mut().unwrap();
        dsp::slice::equilibrium(buffer);
        patch.audio_requested(buffer, SAMPLE_HZ);

        let last_time = prev_time.unwrap_or(time.current);
        let dt = time.current - last_time;
        timer -= dt;
        prev_time = Some(time.current);

        // Traverse inputs or outputs of a node with the following pattern.
        let mut inputs = patch.inputs(master);
        while let Some(input_idx) = inputs.next_node(&patch) {
            if let DspNode::Oscillator(_, ref mut pitch, _) = patch[input_idx] {
                // Pitch down our oscillators for fun.
                *pitch -= 0.1;
            }
        }

        if timer >= 0.0 {
            pa::Continue
        } else {
            pa::Complete
        }
    };

    // Construct PortAudio and the stream.
    let pa = pa::PortAudio::new()?;
    let settings =
        pa.default_output_stream_settings::<Output>(CHANNELS as i32, SAMPLE_HZ, FRAMES)?;
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;

    // Wait for our stream to finish.
    while let true = stream.is_active()? {
        std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}

/// Our type for which we will implement the `Dsp` trait.
#[derive(Debug)]
pub enum DspNode {
    /// Synth will be our demonstration of a master GraphNode.
    Synth,
    /// Oscillator will be our generator type of node, meaning that we will override
    /// the way it provides audio via its `audio_requested` method.
    Oscillator(Phase, Frequency, Volume),
}

impl Node<[Output; CHANNELS]> for DspNode {
    /// Here we'll override the audio_requested method and generate a sine wave.
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            DspNode::Synth => (),
            DspNode::Oscillator(ref mut phase, frequency, volume) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let val = sine_wave(*phase, volume);
                    *phase += frequency / sample_hz;
                    Frame::from_fn(|_| val)
                });
            }
        }
    }
}

/// Return a sine wave for the given phase.
fn sine_wave<S: Sample>(phase: Phase, volume: Volume) -> S
where
    S: Sample + FromSample<f32>,
{
    use std::f64::consts::PI;
    ((phase * PI * 2.0).sin() as f32 * volume).to_sample::<S>()
}

#[cfg(target_os = "linux")]
pub fn event_loop<F, N>(ipc_in: File, ipc_client: File, patch: Graph<F, N>, master: NodeIndex) -> Result<(), Box<error::Error>> 
    where F: dsp::Frame, N: DspNode {
    
    // Get audio devices
    let (audio_dev, rate) = open_audio_dev()?;

    // Get midi devices
    let midi_dev = open_midi_dev()?;
    let mut midi_input = midi_dev.input();

    // Create an array of file descriptors to poll
    let mut fds = audio_dev.get()?;
    fds.append(&mut (&midi_dev, Some(alsa::Direction::Capture)).get()?); 
    
    // Use direct-mode memory mapping for minimum overhead
    let mut mmap = audio_dev.direct_mmap_playback::<SF>();
    
    // if direct-mode unavailable, use mmap emulation instead
    let mut io = if mmap.is_err() {
        Some(audio_dev.io_i16()?)
    } else { None };

    loop {
        if let Ok(ref mut mmap) = mmap {
            if write_samples_direct(&audio_dev, mmap, &mut root)? { continue; }
        } else if let Some(ref mut io) = io {
            if write_samples_io(&audio_dev, io, &mut root)? { continue; }
        }

        if read_midi_event(&mut midi_input, &mut root.synths[0])? { continue; }

        let mut buf: String = String::new();
        ipc_in.read_to_string(&mut buf);
        match &buf[..] {
            "OPEN_PROJECT" => { println!("OPEN"); },

            "PLAY" => { root.timeline.playing = true; },
            "STOP" => { root.timeline.playing = false; },

            "C1_ON" =>  { root.synths[0].add_note(69, 0.5); },
            "C1#_ON" => { root.synths[0].add_note(70, 0.5); },
            "D1_ON" => { root.synths[0].add_note(71, 0.5); },
            "D1#_ON" => { root.synths[0].add_note(72, 0.5); },
            "E1_ON" => { root.synths[0].add_note(73, 0.5); },
            "F1_ON" => { root.synths[0].add_note(74, 0.5); },
            "F1#_ON" => { root.synths[0].add_note(75, 0.5); },
            "G1_ON" => { root.synths[0].add_note(76, 0.5); },
            "G1#_ON" => { root.synths[0].add_note(77, 0.5); },
            "A1_ON" => { root.synths[0].add_note(78, 0.5); },
            "A1#_ON" => { root.synths[0].add_note(79, 0.5); },
            "B1_ON" => { root.synths[0].add_note(80, 0.5); },
            "C2_ON" =>  { root.synths[0].add_note(81, 0.5); },
            "C2#_ON" => { root.synths[0].add_note(82, 0.5); },
            "D2_ON" => { root.synths[0].add_note(83, 0.5); },
            "D2#_ON" => { root.synths[0].add_note(84, 0.5); },
            "E2_ON" => { root.synths[0].add_note(85, 0.5); },
            "F2_ON" => { root.synths[0].add_note(86, 0.5); },
            "F2#_ON" => { root.synths[0].add_note(87, 0.5); },
            "G2_ON" => { root.synths[0].add_note(88, 0.5); },
            "G2#_ON" => { root.synths[0].add_note(89, 0.5); },
            "A2_ON" => { root.synths[0].add_note(90, 0.5); },
            "A2#_ON" => { root.synths[0].add_note(91, 0.5); },
            "B2_ON" => { root.synths[0].add_note(92, 0.5); },
            "C3_ON" =>  { root.synths[0].add_note(93, 0.5); },
            "C3#_ON" => { root.synths[0].add_note(94, 0.5); },
            "D3_ON" => { root.synths[0].add_note(95, 0.5); },
            "D3#_ON" => { root.synths[0].add_note(96, 0.5); },
            "E3_ON" => { root.synths[0].add_note(97, 0.5); },
            "F3_ON" => { root.synths[0].add_note(98, 0.5); },
            "F3#_ON" => { root.synths[0].add_note(99, 0.5); },
            "G3_ON" => { root.synths[0].add_note(100, 0.5); },
            "G3#_ON" => { root.synths[0].add_note(101, 0.5); },
            "A3_ON" => { root.synths[0].add_note(102, 0.5); },
            "A3#_ON" => { root.synths[0].add_note(103, 0.5); },
            "B3_ON" => { root.synths[0].add_note(104, 0.5); },
            "C1_OFF" =>  { root.synths[0].remove_note(69); },
            "C1#_OFF" => { root.synths[0].remove_note(70); },
            "D1_OFF" => { root.synths[0].remove_note(71); },
            "D1#_OFF" => { root.synths[0].remove_note(72); },
            "E1_OFF" => { root.synths[0].remove_note(73); },
            "F1_OFF" => { root.synths[0].remove_note(74); },
            "F1#_OFF" => { root.synths[0].remove_note(75); },
            "G1_OFF" => { root.synths[0].remove_note(76); },
            "G1#_OFF" => { root.synths[0].remove_note(77); },
            "A1_OFF" => { root.synths[0].remove_note(78); },
            "A1#_OFF" => { root.synths[0].remove_note(79); },
            "B1_OFF" => { root.synths[0].remove_note(80); },
            "C2_OFF" =>  { root.synths[0].remove_note(81); },
            "C2#_OFF" => { root.synths[0].remove_note(82); },
            "D2_OFF" => { root.synths[0].remove_note(83); },
            "D2#_OFF" => { root.synths[0].remove_note(84); },
            "E2_OFF" => { root.synths[0].remove_note(85); },
            "F2_OFF" => { root.synths[0].remove_note(86); },
            "F2#_OFF" => { root.synths[0].remove_note(87); },
            "G2_OFF" => { root.synths[0].remove_note(88); },
            "G2#_OFF" => { root.synths[0].remove_note(89); },
            "A2_OFF" => { root.synths[0].remove_note(90); },
            "A2#_OFF" => { root.synths[0].remove_note(91); },
            "B2_OFF" => { root.synths[0].remove_note(92); },
            "C3_OFF" =>  { root.synths[0].remove_note(93); },
            "C3#_OFF" => { root.synths[0].remove_note(94); },
            "D3_OFF" => { root.synths[0].remove_note(95); },
            "D3#_OFF" => { root.synths[0].remove_note(96); },
            "E3_OFF" => { root.synths[0].remove_note(97); },
            "F3_OFF" => { root.synths[0].remove_note(98); },
            "F3#_OFF" => { root.synths[0].remove_note(99); },
            "G3_OFF" => { root.synths[0].remove_note(100); },
            "G3#_OFF" => { root.synths[0].remove_note(101); },
            "A3_OFF" => { root.synths[0].remove_note(102); },
            "A3#_OFF" => { root.synths[0].remove_note(103); },
            "B3_OFF" => { root.synths[0].remove_note(104); },
            _ => {}
        }

        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}