extern crate termion;

use std::{time, thread};

use termion::{clear, color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, BufReader};
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

struct MenuState<'a> {
    motd: &'a str,
    projects: Vec<&'a str>,
}

fn main() -> std::io::Result<()> {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let asset_file = File::open("src/assets/logo.txt").unwrap();

    let mut buf_reader = BufReader::new(asset_file);
    let mut asset_str = String::new();

    buf_reader.read_to_string(&mut asset_str).unwrap();

    let state: MenuState = MenuState {
        motd: "It's Fun!",
        projects: vec![
            "tinytoes.xml",
            "heyo!!.xml",
            "tinytoes.xml",
            "heyo!!.xml",
        ],
    };    

    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    stdout.flush().unwrap();

    for (i, line) in asset_str.lines().enumerate() {
        write!(stdout,
            "{}{}{}{}",
            cursor::Goto(1, (i as u16)+1),
            color::Fg(color::Red),
            color::Bg(color::Black),
            line).unwrap();
    }

    stdout.flush().unwrap();

    for c in stdin.keys() {

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}