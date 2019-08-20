use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use cursive::{Cursive, Printer};
use cursive::theme::{BaseColor, Color, ColorStyle, Effect, Theme, BorderStyle, Palette};
use cursive::vec::Vec2;

#[derive(Debug)]
pub struct Splash {
    message: String,
    selected: bool,
    text: String
}

// (x, y)
const MARGIN: (usize, usize) = (0, 0);

impl Splash {
    pub fn new(message: &str) -> Self {

        let logo_file = match File::open("src/assets/logo.txt") {
            Ok(f)  => f,
            Err(e) => panic!("{}",  e)
        };

        let mut buf_reader = BufReader::new(logo_file);
        let mut logo = String::new();
        buf_reader.read_to_string(&mut logo);

        Splash {
            message: message.to_string(),
            selected: false,
            text: logo,
        }
    }
}

impl cursive::view::View for Splash {
    fn draw(&self, printer: &Printer) {

        let style = ColorStyle::new(
            Color::Dark(BaseColor::Red), 
            Color::Dark(BaseColor::Black)
        );

        for (i, line) in self.text.lines().enumerate() {
            printer.with_color(style, 
                |printer| printer.print(( (MARGIN.0/2), i + (MARGIN.1/2)), &line.to_string()),
            )
        }

        let splashY: usize = self.text.lines().count() + 1;
        printer.with_color(style, 
            |printer| printer.print((0, splashY), &self.message),
        )

    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let width: usize = self.text.lines().nth(0).unwrap().len() + MARGIN.0;
        let height: usize = match self.message.len() {
            0 => self.text.lines().count() + MARGIN.1,
            _ => self.text.lines().count() + MARGIN.1 + 2
        };
        Vec2::new(width, height)
    }
}
