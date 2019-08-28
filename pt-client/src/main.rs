extern crate termion;

use termion::{clear, color, cursor, terminal_size};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, Stdout, stdin, BufReader};
use std::io::prelude::*;
use std::fs::{File};

/*
mod utils;
mod components;
mod views;
mod core;

use views::{Home};
use utils::{HomeState, read_document, write_document};
*/

// const HOME_DIR = "/usr/local/palit/" // PROD
const HOME_DIR: &str = "storage/";

#[derive(Clone, Debug)]
struct MenuState {
    motd: String,
    projects: Vec<String>,
    focus: usize,
}

enum Action {
    Up,
    Down,
    Select,
    Noop,
}

fn reducer(state: MenuState, action: Action) -> MenuState {
    MenuState {
        motd: state.motd,
        focus: match action {
            Action::Up => state.focus+1,
            Action::Down => state.focus-1,
            _ => state.focus,
        },
        projects: state.projects,
    }
}

fn render(state: MenuState, x: u16, y: u16, mut out: RawTerminal<Stdout>, asset_str: String) -> RawTerminal<Stdout> {

    // Clear Screen
    write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
    out.flush().unwrap();

    for (i, line) in asset_str.lines().enumerate() {
        write!(out, "{}{}{}",
            cursor::Goto(x+1, (i as u16)+y+1),
            color::Fg(color::White),
            line).unwrap();
    }
    out.flush().unwrap();

    write!(out, "{}{}",
        cursor::Goto(1,1),
        state.focus).unwrap();
    out.flush().unwrap();

    out
}

fn main() -> std::io::Result<()> {

    // Configure stdin and raw_mode stdout
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Load logo asset
    let asset_file = File::open("src/assets/logo.txt").unwrap();
    let mut buf_reader = BufReader::new(asset_file);
    let mut asset_str = String::new();
    buf_reader.read_to_string(&mut asset_str).unwrap();

    // Calculate center position
    let size: (u16, u16) = terminal_size().unwrap();
    let mut max_len: u16 = 0; 
    for line in asset_str.lines() {
        let len = line.len();
        if len as u16 > max_len {
            max_len = len as u16;
        }
    }
    let asset_x = (size.0 / 2) - (max_len / 2);

    // Initialize State
    let mut state: MenuState = MenuState {
        motd: "It's Fun!".to_string(),
        projects: vec![
            "tinytoes.xml".to_string(),
            "heyo!!.xml".to_string(),
            "tinytoes.xml".to_string(),
            "heyo!!.xml".to_string(),
        ],
        focus: 100,
    };

    stdout = render(state.clone(), asset_x, 3, stdout, asset_str.clone());

    // Loops until break
    for c in stdin.keys() {

        stdout = render(state.clone(), asset_x, 3, stdout, asset_str.clone());

        let action: Action = match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            Key::Right => Action::Select,
            _ => Action::Noop,
        };

        state = reducer(state.clone(), action);
    }

    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}