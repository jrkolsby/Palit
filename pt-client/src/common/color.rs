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
    Transparent,
}

pub fn write_bg(mut out: RawTerminal<Stdout>, c: Color) -> RawTerminal<Stdout> {
    match c {
        Color::Red => write!(out, "{}", color::Bg(color::Red)),
        Color::Pink => write!(out, "{}", color::Bg(color::Magenta)),
        Color::Blue => write!(out, "{}", color::Bg(color::Blue)),
        Color::Green => write!(out, "{}", color::Bg(color::Green)),
        Color::Yellow => write!(out, "{}", color::Bg(color::Yellow)),
        Color::White => write!(out, "{}", color::Bg(color::White)),
        Color::Black => write!(out, "{}", color::Bg(color::Black)),
        _ => Ok(())
    };
    out
}

pub fn write_fg(mut out: RawTerminal<Stdout>, c: Color) -> RawTerminal<Stdout> {
    match c {
        Color::Red => write!(out, "{}", color::Fg(color::Red)),
        Color::Pink => write!(out, "{}", color::Fg(color::Magenta)),
        Color::Blue => write!(out, "{}", color::Fg(color::Blue)),
        Color::Green => write!(out, "{}", color::Fg(color::Green)),
        Color::Yellow => write!(out, "{}", color::Fg(color::Yellow)),
        Color::White => write!(out, "{}", color::Fg(color::White)),
        Color::Black => write!(out, "{}", color::Fg(color::Black)),
        _ => Ok(())
    };
    out
}