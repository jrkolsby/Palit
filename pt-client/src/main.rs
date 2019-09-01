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
    for layer in (*layers).iter() {
        stdout = layer.render(stdout);
        // Only render until first fullscreen layer 
        if layer.alpha() { break; }
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

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    stdout = render(stdout, &layers);
    stdout.flush().unwrap();

    // Loops until break
    for c in stdin.keys() {

        let action: Action = match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            Key::Left => Action::Left,
            Key::Right => Action::Right,
            _ => Action::Noop,
        };

        // Dispatch action to front layer and match talkback action
        match layers[0].dispatch(action) {
            Action::OpenProject(s) => println!("OPEN {}", s),
            _ => {}
        };

        write!(stdout, "{}", clear::All).unwrap();
        stdout.flush().unwrap();

        stdout = render(stdout, &layers);
    }

    // CLEAN UP
    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}