extern crate termion;

use std::io::{Write, stdout, stdin};

use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};

mod components; // NEED THIS IN MAIN FOR OTHER MODULES TO FIND IT!!!
mod common;
mod views;

use views::{Home, Timeline};

use common::{Action, Region, Asset, Track};

fn main() -> std::io::Result<()> {

    // Configure stdin and raw_mode stdout
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut home: Home = Home::new();
    let mut timeline: Timeline = Timeline::new();

    let mut enableHome = true;
    let mut enableTimeline = false;

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    if enableHome { stdout = home.render(stdout); }
    if enableTimeline { stdout = timeline.render(stdout); }

    stdout.flush().unwrap();

    // Loops until break
    for c in stdin.keys() {

        let action: Action = match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            Key::Right => {
                enableHome = false;
                enableTimeline = true;
                Action::Noop
            },
            Key::Left => {
                enableHome = true;
                enableTimeline = false;
                Action::Noop
            }
            _ => Action::Noop,
        };

        if enableHome { home.dispatch(action.clone()); }
        if enableTimeline { timeline.dispatch(action.clone()); }

        write!(stdout, "{}", clear::All).unwrap();
        stdout.flush().unwrap();

        if enableHome { stdout = home.render(stdout); }
        if enableTimeline { stdout = timeline.render(stdout); }
    }

    // CLEAN UP
    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}