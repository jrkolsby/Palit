extern crate libc;
extern crate termion;
extern crate linux_raw_input_rs;

use std::io::{BufReader, Write, Stdout, stdout, stdin};
use std::io::prelude::*;
use std::fs::{OpenOptions, read_to_string};
use std::os::unix::fs::OpenOptionsExt;

use termion::{clear, cursor, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};

use linux_raw_input_rs::{InputReader, get_input_devices};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::input::EventType;

// NOTE: These need to be here
mod views;
mod common;
mod components; 

use views::{Layer, Home, Timeline, Help, Title};

use common::{Action};

fn render(mut stdout: RawTerminal<Stdout>, layers: &Vec<Box<Layer>>) -> RawTerminal<Stdout> {
    /*
        LAYERS:
        4:   ---    <- End render here
        3:  -----
        2: -------  <- Start render here (!alpha)
        1:   ---
        0: -------  <- Home
    */
    // Determine bottom layer
    let mut bottom = layers.len()-1;
    for layer in (*layers).iter().rev() {
        if layer.alpha() { bottom -= 1 }
        else { break }
    };
    for i in bottom..(*layers).len() {
        stdout = layers[i].render(stdout);
    }
    stdout
}

fn main() -> std::io::Result<()> {

    // Configure pt-sound IPC
    println!("Awaiting pt-sound...");
    // Blocked by pt-sound reader
    let mut ipc_out = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-client").unwrap();

    let mut ipc_in = OpenOptions::new()
        .custom_flags(libc::O_NONBLOCK)
	.read(true)
	.open("/tmp/pt-sound").unwrap();
    let mut ipc_in_buf = String::new();

    // Configure raw_mode stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Configure keyboard input
    let device_path : String = get_input_devices().iter().nth(0).expect("Problem with iterator").to_string();
    let mut reader = InputReader::new(device_path);

    // Configure margins and sizes
    let size: (u16, u16) = terminal_size().unwrap();

    // Configure UI layers
    let mut layers: Vec<Box<Layer>> = Vec::new();
    layers.push(Box::new(Home::new(1, 1, size.0, size.1)));

    // Hide cursor and clear screen
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // Initial Render
    stdout = render(stdout, &layers);
    stdout.flush().unwrap();

    // MAIN LOOP
    loop {

	// Get keyboard state
        let input = reader.current_state();
	let event = (input.event_type(), input.get_key());

        ipc_in.read_to_string(&mut ipc_in_buf).unwrap();
        if ipc_in_buf.len() > 0 {
            eprintln!("ipc_in {}", ipc_in_buf);
        }

        // Map keypress to Action
        let action: Action = match event {
            (EventType::Push, Keys::KEY_Q) => break,
            (EventType::Push, Keys::KEY_1) => Action::Help,
            (EventType::Push, Keys::KEY_2) => Action::Back,
            (EventType::Push, Keys::KEY_LEFTBRACE) => Action::Play,
            (EventType::Push, Keys::KEY_RIGHTBRACE) => Action::Stop,

            (EventType::Push, Keys::KEY_M) => Action::SelectG,
            (EventType::Push, Keys::KEY_R) => Action::SelectY,
            (EventType::Push, Keys::KEY_V) => Action::SelectP,
            (EventType::Push, Keys::KEY_I) => Action::SelectB,
            (EventType::Push, Keys::KEY_SPACE) => Action::SelectR,

            (EventType::Push, Keys::KEY_A) => Action::NoteC1,
            (EventType::Push, Keys::KEY_S) => Action::NoteD1,
            (EventType::Push, Keys::KEY_D) => Action::NoteE1,
            (EventType::Push, Keys::KEY_F) => Action::NoteF1,
            (EventType::Push, Keys::KEY_G) => Action::NoteG1,
            (EventType::Push, Keys::KEY_H) => Action::NoteA1,
            (EventType::Push, Keys::KEY_J) => Action::NoteB1,
            (EventType::Push, Keys::KEY_K) => Action::NoteC2,
            (EventType::Push, Keys::KEY_L) => Action::NoteD2,
            (EventType::Push, Keys::KEY_UP) => Action::Up,
            (EventType::Push, Keys::KEY_DOWN) => Action::Down,
            (EventType::Push, Keys::KEY_LEFT) => Action::Left,
            (EventType::Push, Keys::KEY_RIGHT) => Action::Right,

            (_, _) => Action::Noop,
        };

        // Catch root actions else dispatch to top layer
        let talkback: Action = match action {
            Action::Play => { ipc_out.write(b"PLAY"); Action::Noop }
            Action::Stop => { ipc_out.write(b"STOP"); Action::Noop }
            Action::Help => { 
                layers.push(Box::new(Help::new(10, 10, 44, 15))); 
                Action::Noop
            },
            Action::Back => { 
                layers.pop(); 
                Action::Noop
            }, 
            _ => {
                // Dispatch action to front layer and match talkback action
                let target = layers.last_mut().unwrap();
                target.dispatch(action)
            }
        };

        // Dispatch root action if returned from layer
        match talkback {
            Action::InputTitle => {
                layers.push(Box::new(Title::new(23, 5, 36, 23)));
            },
            Action::CreateProject(title) => {
                ipc_out.write(b"NEW_PROJECT\n");
                layers.push(Box::new(Timeline::new(1, 1, size.0, size.1, title)));
            },
            Action::OpenProject(title) => {
                ipc_out.write(b"OPEN_PROJECT\n");
                ipc_out.write(title.as_bytes());
                layers.push(Box::new(Timeline::new(1, 1, size.0, size.1, title)));
            },
            Action::Back => { layers.pop(); }, 
            Action::Pepper => {
                layers.push(Box::new(Help::new(10, 10, 44, 15))); 
            },
            _ => {}
        };	

        // Clears screen
        write!(stdout, "{}", clear::All).unwrap();
        stdout.flush().unwrap();

        // Renders layers
        stdout = render(stdout, &layers);
    }

    // CLEAN UP
    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}
