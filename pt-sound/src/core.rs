extern crate sample;
extern crate portaudio;

use std::{iter, error};
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;

#[cfg(target_os = "linux")]
extern crate alsa;
#[cfg(target_os = "linux")]
use alsa::{seq, pcm, PollDescriptors};
#[cfg(target_os = "linux")]
use alsa::pcm::State;

use dsp::{sample::ToFrameSliceMut, NodeIndex, Frame, FromSample};
use dsp::{Outputs, Graph, Node, Sample, Walker};
use dsp::daggy::petgraph::Bfs;

#[cfg(target_os = "macos")]
use portaudio as pa;

use sample::signal;

use crate::midi::{open_midi_dev, read_midi_event, connect_midi_source_ports};
use crate::action::Action;
use crate::synth;

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
pub fn event_loop<F: 'static>(
        mut ipc_in: File, 
        mut ipc_client: File, 
        mut patch: Graph<[Output; CHANNELS], Module>, 
        midi_keys: NodeIndex,
        keys: NodeIndex,
        mut dispatch_f: F) -> Result<(), Box<error::Error>> 
    where F: FnMut(Action) -> Action {

    // The callback we'll use to pass to the Stream. It will request audio from our dsp_graph.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, time, .. }| {

        let ipc_actions: Vec<Action> = ipc_action(&ipc_in);

        for action in ipc_actions.iter() {
            match action {
                // Can mutate graph here, before the walk below
                Action::AddRoute(nid_o, oid, nid_i, iid) => {
                    println!("Mutating graph!");
                },
                Action::DeleteRoute(eid) => {
                    println!("deleting edge!");
                },
                // Give action directly to indexed node
                Action::SetParam(nid, _, _) => {
                    // TODO: Make sure that node index exists!
                    // ... be careful when nodes are removed that the
                    // ... indicies are shifted accordingly
                    let id = NodeIndex::new(*nid);
                    patch[id].dispatch(action.clone());
                },
                // Give notes to the outputs of keyboard
                Action::NoteOn(_,_) | Action::NoteOff(_) => {
                    patch[keys].dispatch(action.clone());
                },
                Action::Exit => { return pa::Complete },
                _ => { println!("Got bad action, {:?}", action);}
            };
        }

        // Nodes dispatch actions to its ins, outs, or to client. Midi signals
        // ... must travel opposite the direciton of audio in an acyclic graph
        let mut walk = patch.visit_order_rev();
        while let Some(n) = walk.next(&patch) {
            let (out_d, in_d, client_d) = patch[n].dispatch_requested();
            if let Some(mut out_a) = out_d {
                let mut outs = patch.outputs(n);
                while let Some(oid) = outs.next_node(&patch) {
                    while let Some(a) = out_a.pop() {
                        patch[oid].dispatch(a.clone());
                    }
                }
            }
            if let Some(mut in_a) = in_d {
                let mut ins = patch.inputs(n);
                while let Some(iid) = ins.next_node(&patch) {
                    while let Some(a) = in_a.pop() {
                        patch[iid].dispatch(a.clone());
                    }
                }
            }
            if let Some(client_a) = client_d {
                for a in client_a.iter() {
                    // DISPATCH ACTION TO ipc_client
                    //patch[oid].dispatch(a.clone());
                }
            }
        }

        let buffer: &mut [[Output; CHANNELS]] = buffer.to_frame_slice_mut().unwrap();
        dsp::slice::equilibrium(buffer);
        patch.audio_requested(buffer, SAMPLE_HZ);

        pa::Continue
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

// Our type for which we will implement the `Dsp` trait.
pub enum Module {
    /// Synth will be our demonstration of a master GraphNode.
    Master,
    /// Oscillator will be our generator type of node, meaning that we will override
    /// the way it provides audio via its `audio_requested` method.
    Oscillator(Phase, Frequency, Volume),
    Synth(synth::Store),
    Passthru(Vec<Action>),
    DebugKeys(Vec<Action>, Vec<Action>, u16),
}

impl Module {
    pub fn dispatch(&mut self, a: Action) {
        match *self {
            Module::Master => {}
            Module::Passthru(ref mut queue) => { queue.push(a.clone()) }
            Module::DebugKeys(ref mut onqueue, _, _) => { onqueue.push(a.clone()); }
            Module::Synth(ref mut store) => synth::dispatch(store, a.clone()),
            _ => {}
        };
    }
    pub fn dispatch_requested(&mut self) -> (
            Option<Vec<Action>>, // Actions for outputs
            Option<Vec<Action>>, // Actions for inputs
            Option<Vec<Action>> // Actions for client
        ) {

        match *self {
            Module::Passthru(ref mut queue) => {
                let carry = queue.clone();
                queue.clear();
                return (Some(carry), None, None)
            },
            Module::DebugKeys(ref mut onqueue, ref mut offqueue, ref mut timer) => {
                let carry = onqueue.clone();
                while let Some(note) = onqueue.pop() {
                    offqueue.push(match note {
                        Action::NoteOn(n, _) => Action::NoteOff(n),
                        _ => Action::Noop,
                    });
                }
                if *timer == 0 {
                    *timer = 48000;
                    return (Some(offqueue.clone()), None, None)
                } else {
                    return (Some(carry), None, None)
                }
            }
            Module::Master => (None, None, None), // TODO: give master levels to client
            _ => (None, None, None)
        }
    }
}

impl Node<[Output; CHANNELS]> for Module {
    // Override the audio_requested method and compute PCM audio
    fn audio_requested(&mut self, buffer: &mut [[Output; CHANNELS]], sample_hz: f64) {
        match *self {
            Module::Master => (),
            Module::Oscillator(ref mut phase, frequency, volume) => {
                dsp::slice::map_in_place(buffer, |_| {
                    let val = sine_wave(*phase, volume);
                    *phase += frequency / sample_hz;
                    Frame::from_fn(|_| val)
                });
            },
            Module::Synth(ref mut store) => {
                dsp::slice::map_in_place(buffer, |_| {
                    Frame::from_fn(|_| synth::compute(store))
                });
            },
            // Modules which aren't sound-producing can still implement audio_requested
            // ... to keep time, such as envelopes or arpeggiators
            Module::DebugKeys(_, _, ref mut timer) => {
                if *timer > 0 { 
                    *timer = *timer - buffer.len() as u16; 
                }
            },
            _ => ()
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

fn ipc_action(mut ipc_in: &File) -> Vec<Action> {
    let mut buf: String = String::new();
    ipc_in.read_to_string(&mut buf);
    let mut ipc_iter = buf.split(" ");

    let mut events: Vec<Action> = Vec::new();

    while let Some(action_raw) = ipc_iter.next() {
        let argv: Vec<&str> = action_raw.split(":").collect();

        let action = match argv[0] {

            "PLAY" => Action::Play,
            "STOP" => Action::Stop,

            "NOTE_ON" => Action::NoteOn(argv[1].parse::<u8>().unwrap(), 
                                        argv[2].parse::<f64>().unwrap()),

            "NOTE_OFF" => Action::NoteOff(argv[1].parse::<u8>().unwrap()),

            "EXIT" => Action::Exit,

            _ => Action::Noop,
        };

        match action {
            Action::Noop => {},
            _ => { events.push(action); }
        };
    };

    events
}

#[cfg(target_os = "linux")]
pub fn event_loop(
        mut ipc_in: File, 
        mut ipc_client: File, 
        mut patch: Graph<[Output; CHANNELS], Module>) -> Result<(), Box<error::Error>> {
    
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

        let a: Action = ipc_action(&ipc_in);

        // Nothing to do, let's sleep until woken up by the kernel.
        alsa::poll::poll(&mut fds, 100)?;
    }
}