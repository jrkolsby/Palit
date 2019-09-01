extern crate termion;

use std::io::{Write, Stdout, stdout, stdin};

use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// NOTE: These need to be here
mod components; 
mod common;
mod views;

use views::{Layer, Home, Timeline};

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

    // Configure stdin and raw_mode stdout
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Configure UI layers
    let mut layers: Vec<Box<Layer>> = Vec::new();
    layers.push(Box::new(Home::new()));

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
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            Key::Left => Action::Left,
            Key::Right => Action::Right,
            _ => Action::Noop,
        };

        // Dispatch action to front layer and match talkback action
        let target = layers.last_mut().unwrap();
        match target.dispatch(action) {
            Action::OpenProject(s) => {
                eprintln!("OPEN {}", s);
                layers.push(Box::new(Timeline::new()));
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