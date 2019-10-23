extern crate libc;
extern crate termion;

use std::io::{BufReader, Write, Stdout, stdout, stdin};
use std::io::prelude::*;
use std::fs::{OpenOptions, read_to_string, File};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;

use termion::{clear, cursor, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};

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

fn ipc_action(mut ipc_in: &File) -> Vec<Action> {
    let mut buf: String = String::new();
    ipc_in.read_to_string(&mut buf);
    let mut ipc_iter = buf.split(" ");

    let mut events: Vec<Action> = Vec::new();

    while let Some(action_raw) = ipc_iter.next() {
        let argv: Vec<&str> = action_raw.split(":").collect();

        let action = match argv[0] {
            "TICK" => Action::Tick,

            "?" => Action::Noop,

            "EXIT" => break,
            "1" => Action::Help,
            "2" => Action::Back,

            "PLAY" => Action::Play,
            "STOP" => Action::Stop,

            "M" => Action::SelectG,
            "R" => Action::SelectY,
            "V" => Action::SelectP,
            "I" => Action::SelectB,
            "SPC" => Action::SelectR,

            "A" => Action::NoteC1,
            "S" => Action::NoteD1,
            "D" => Action::NoteE1,
            "F" => Action::NoteF1,
            "G" => Action::NoteG1,
            "H" => Action::NoteA1,
            "J" => Action::NoteB1,
            "K" => Action::NoteC1,
            "L" => Action::NoteD1,

            "UP" => Action::Up,
            "DN" => Action::Down,
            "LT" => Action::Left,
            "RT" => Action::Right,

            _ => { Action::Noop },
        };

        match action {
            Action::Noop => {},
            _ => { events.push(action); }
        };
    };

    events
}
fn main() -> std::io::Result<()> {

    // Public action fifo /tmp/pt-client
    let mut ipc_in = OpenOptions::new()
        .custom_flags(libc::O_NONBLOCK)
	    .read(true)
	    .open("/tmp/pt-client").unwrap();

    // Blocked by pt-sound reader
    // If a process runs and nobody is around to hear it,
    // should it really continue? 
    println!("Waiting for pt-sound...");

    let mut ipc_sound = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-sound").unwrap();

    // Configure raw_mode stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Configure input polling array
    let in_src = CString::new("/tmp/pt-client").unwrap();
    let mut fds: Vec<libc::pollfd> = unsafe {vec![
        libc::pollfd { 
            fd: libc::open(in_src.as_ptr(), libc::O_RDONLY),
            events: libc::POLLIN,
            revents: 0,
        },
    ]};

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

    // Action queue
    let mut events: Vec<Action> = Vec::new();

    // MAIN LOOP
    loop {

        unsafe{
            libc::poll(&mut fds[0] as *mut libc::pollfd, fds.len() as libc::nfds_t, 100);
        }

        // If anybody else closes the pipe, halt TODO: Throw error
        if fds[0].revents & libc::POLLHUP == libc::POLLHUP { break; }

        let mut events: Vec<Action> = if fds[0].revents > 0 {
            ipc_action(&mut ipc_in)
        } else { continue; };

        while let Some(next) = events.pop() {
            // Execute toplevel actions, capture default from view
            let default: Action = match next {
                Action::Play => { ipc_sound.write(b"PLAY "); Action::Noop }
                Action::Stop => { ipc_sound.write(b"STOP "); Action::Noop }
                Action::Help => { 
                    layers.push(Box::new(Help::new(10, 10, 44, 15))); 
                    Action::Noop
                },
                Action::Back => { 
                    layers.pop(); 
                    Action::Noop
                }, 
                // Dispatch toplevel action to front layer
                a => {
                    let target = layers.last_mut().unwrap();
                    target.dispatch(a)
                }
            };

            // capture default action if returned from layer
            match default {
                Action::InputTitle => {
                    layers.push(Box::new(Title::new(23, 5, 36, 23)));
                },
                Action::CreateProject(title) => {
                    ipc_sound.write(format!("NEW_PROJECT:{} ", title).as_bytes());
                    layers.push(Box::new(Timeline::new(1, 1, size.0, size.1, title)));
                },
                Action::OpenProject(title) => {
                    ipc_sound.write(format!("OPEN_PROJECT:{} ", title).as_bytes());
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

    }

    // CLEAN UP
    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}
