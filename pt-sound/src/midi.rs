#[cfg(target_os = "linux")]
extern crate alsa;
extern crate sample;

#[cfg(target_os = "linux")]
use alsa::{seq, pcm};
use std::{iter, error};
use std::ffi::CString;
use sample::signal;
use libcommon::Action;

#[cfg(target_os = "linux")]
pub fn connect_midi_source_ports(s: &alsa::Seq, our_port: i32) -> Result<(), Box<error::Error>> {
    // Iterate over clients and clients' ports
    let our_id = s.client_id()?;
    let ci = seq::ClientIter::new(&s);
    for client in ci {
        if client.get_client() == our_id { continue; } // Skip ourselves
        let pi = seq::PortIter::new(&s, client.get_client());
        for port in pi {
            let caps = port.get_capability();

            // Check that it's a normal input port
            if !caps.contains(seq::READ) || !caps.contains(seq::SUBS_READ) { continue; }
            if !port.get_type().contains(seq::MIDI_GENERIC) { continue; }

            // Connect source and dest ports
            let subs = seq::PortSubscribe::empty()?;
            subs.set_sender(seq::Addr { client: port.get_client(), port: port.get_port() });
            subs.set_dest(seq::Addr { client: our_id, port: our_port });
            println!("Reading from midi input {:?}", port);
            s.subscribe_port(&subs)?;
        }
    }

    Ok(())
} 

#[cfg(target_os = "linux")]
pub fn open_midi_dev() -> Result<alsa::Seq, Box<error::Error>> {
    // Open the sequencer.
    let s = alsa::Seq::open(None, Some(alsa::Direction::Capture), true)?;
    let cstr = CString::new("rust_synth_example").unwrap();
    s.set_client_name(&cstr)?;

    // Create a destination port we can read from
    let mut dinfo = seq::PortInfo::empty().unwrap();
    dinfo.set_capability(seq::WRITE | seq::SUBS_WRITE);
    dinfo.set_type(seq::MIDI_GENERIC | seq::APPLICATION);
    dinfo.set_name(&cstr);
    s.create_port(&dinfo).unwrap();
    let dport = dinfo.get_port();

    // source ports should ideally be configurable, but right now we're just reading them all.
    connect_midi_source_ports(&s, dport)?;

    Ok(s)
}

#[cfg(target_os = "macos")]
pub fn open_midi_dev() -> Result<(), Box<error::Error>> { Ok(()) }

#[cfg(target_os = "macos")]
pub fn read_midi_event() -> Result<(), Box<error::Error>> { Ok(()) }

#[cfg(target_os = "macos")]
pub fn connect_midi_source_ports() -> Result<(), Box<error::Error>> { Ok(()) }

#[cfg(target_os = "linux")]
pub fn read_midi_event(input: &mut seq::Input) -> Result<Option<Action>, Box<error::Error>> {
    if input.event_input_pending(true)? == 0 { return Ok(None); }
    let ev = input.event_input()?;
    // println!("Received: {:?}", ev);
    match ev.get_type() {
        seq::EventType::Noteon => {
            let data: seq::EvNote = ev.get_data().unwrap();
            if data.velocity == 0 {
                Ok(Some(Action::NoteOff(data.note)))
            } else {
                Ok(Some(Action::NoteOn(data.note, f64::from(data.velocity + 64) / 2048.)))
            }
        },
        seq::EventType::Noteoff => {
            let data: seq::EvNote = ev.get_data().unwrap();
            Ok(Some(Action::NoteOff(data.note)))
        },
        seq::EventType::Controller => {
            let data: seq::EvCtrl = ev.get_data().unwrap();
            Ok(Some(Action::SetParam(0, format!("{}", data.param), data.value)))
        },
        _ => Ok(None),
    }
}
