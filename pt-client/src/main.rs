extern crate libc;
extern crate termion;

use std::io::{Write, Stdout, stdout, stdin};
use std::io::prelude::*;
use std::fs::{OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

use termion::{clear, cursor, terminal_size};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// NOTE: These need to be here
mod views;
mod common;
mod components; 

use views::{Layer, Home, Timeline, Help};

use common::{Action, Region, Asset, Track};

// struct State {}

fn back() {} // Pop a layer

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

    // READER MUST OPEN BEFORE WRITER
    let mut ipc_out = OpenOptions::new()
	//.custom_flags(libc::O_NONBLOCK)
	.write(true)
	.open("/tmp/pt-client").unwrap();

    let mut ipc_in = OpenOptions::new()
	.custom_flags(libc::O_NONBLOCK)
	.read(true)
	.open("/tmp/pt-sound").unwrap();

    ipc_out.write(b"HELLO FROM CLIENT");
    let mut buf = String::new();
    ipc_in.read_to_string(&mut buf);
    eprintln!("SOUND: {}", buf);

    // Configure stdin and raw_mode stdout
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Configure margins and sizes
    let size: (u16, u16) = terminal_size().unwrap();

    // Configure UI layers
    let mut layers: Vec<Box<Layer>> = Vec::new();
    layers.push(Box::new(Home::new(0, 3, size.0, size.1)));

    // Hide cursor and clear screen
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // Initial Render
    stdout = render(stdout, &layers);
    stdout.flush().unwrap();

    // Loops until break
    for c in stdin.keys() {

        // Map keypress to Action
        let action: Action = match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('1') => Action::Help,
            Key::Char('2') => Action::Back,
            Key::Char(' ') => Action::SelectR,
            Key::Char('v') => Action::SelectG,
            Key::Char(',') => Action::SelectY,
            Key::Char('t') => Action::SelectP,
            Key::Char('i') => Action::SelectB,
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            Key::Left => Action::Left,
            Key::Right => Action::Right,
            _ => Action::Noop,
        };

        // Dispatch Action and capture talkback
        match action {
            Action::Help => { layers.push(Box::new(Help::new(10, 10, 44, 15))); },
            Action::Back => { layers.pop(); }, 
            _ => {
                // Dispatch action to front layer and match talkback action
                let target = layers.last_mut().unwrap();
                match target.dispatch(action) {
                    Action::OpenProject(s) => {
                        eprintln!("OPEN {}", s);
                        layers.push(Box::new(Timeline::new(0, 3, size.0, size.1)));
                    },
                    Action::Back => { layers.pop(); }, 
                    _ => {}
                };
            }
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
