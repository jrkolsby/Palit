use termion::{color, cursor};
use std::io::{Write, Stdout};
use termion::raw::{RawTerminal};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Pink,
    Blue,
    Green,
    Yellow,
    White,
    Black,
    Beige,
    Transparent,
}

pub fn write_bg(mut out: RawTerminal<Stdout>, c: Color) -> RawTerminal<Stdout> {
    match c {
        Color::Red => write!(out, "{}", color::Bg(color::Red)).unwrap(),
        Color::Pink => write!(out, "{}", color::Bg(color::Magenta)).unwrap(),
        Color::Blue => write!(out, "{}", color::Bg(color::Blue)).unwrap(),
        Color::Green => write!(out, "{}", color::Bg(color::Green)).unwrap(),
        Color::Yellow => write!(out, "{}", color::Bg(color::Yellow)).unwrap(),
        Color::White => write!(out, "{}", color::Bg(color::White)).unwrap(),
        Color::Black => write!(out, "{}", color::Bg(color::Black)).unwrap(),
        Color::Beige => write!(out, "{}", color::Bg(color::LightYellow)).unwrap(),
        Color::Transparent => write!(out, "{}", color::Bg(color::Reset)).unwrap(),
        _ => ()
    };
    out
}

pub fn write_fg(mut out: RawTerminal<Stdout>, c: Color) -> RawTerminal<Stdout> {
    match c {
        Color::Red => write!(out, "{}", color::Fg(color::Red)).unwrap(),
        Color::Pink => write!(out, "{}", color::Fg(color::Magenta)).unwrap(),
        Color::Blue => write!(out, "{}", color::Fg(color::Blue)).unwrap(),
        Color::Green => write!(out, "{}", color::Fg(color::Green)).unwrap(),
        Color::Yellow => write!(out, "{}", color::Fg(color::Yellow)).unwrap(),
        Color::White => write!(out, "{}", color::Fg(color::White)).unwrap(),
        Color::Black => write!(out, "{}", color::Fg(color::Black)).unwrap(),
        Color::Beige => write!(out, "{}", color::Fg(color::LightYellow)).unwrap(),
        _ => ()
    };
    out
}