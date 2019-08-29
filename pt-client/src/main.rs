extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};

mod views;
use views::{Home, HomeAction, Timeline, TimelineAction};

mod components; // NEED THIS IN MAIN FOR OTHER MODULES TO FIND IT!!!
mod common;
use common::{Region, Asset, Track};

/*
mod utils;
mod components;
mod core;

use views::{Home};
use utils::{HomeState, read_document, write_document};
*/

fn main() -> std::io::Result<()> {

    // Configure stdin and raw_mode stdout
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut root: Home = Home::new();
    let mut timeline: Timeline = Timeline::new();

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    stdout = root.render(timeline.render(stdout));
    stdout.flush().unwrap();

    // Loops until break
    for c in stdin.keys() {

        let action: HomeAction = match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => HomeAction::Up,
            Key::Down => HomeAction::Down,
            Key::Right => HomeAction::Select,
            _ => HomeAction::Noop,
        };

        write!(stdout, "{}", clear::All).unwrap();
        root.dispatch(action);

        stdout = root.render(timeline.render(stdout));
        stdout = timeline.render(stdout);
    }

    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}