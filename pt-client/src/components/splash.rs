use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use cursive::Printer;
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::direction::Direction;
use cursive::event::{EventResult, Event, Key};
use cursive::vec::Vec2;

// A splash is a static ASCII component
pub struct Splash {
    message: String,
    selected: bool,
    text: String
}

pub enum SplashAsset {
    Logo,
    Keyboard,
    Guitar,
}

// (x, y)
const MARGIN: (usize, usize) = (0, 0);

impl Splash {
    pub fn new(splash_type: SplashAsset, message: &str) -> Self {

        let asset = match splash_type {
            SplashAsset::Logo => "src/assets/logo.txt",
            SplashAsset::Keyboard => "src/assets/keyboard.txt",
            _ => "src/assets/error.txt"
        };

        let asset_file = match File::open(asset) {
            Ok(f)  => f,
            Err(e) => panic!("{}",  e)
        };

        let mut buf_reader = BufReader::new(asset_file);
        let mut asset_str = String::new();
        match buf_reader.read_to_string(&mut asset_str) {
            Ok(f)  => f,
            Err(e) => panic!("{}",  e)
        };

        Splash {
            message: message.to_string(),
            selected: false,
            text: asset_str,
        }
    }
}

impl cursive::view::View for Splash {
    fn draw(&self, printer: &Printer) {

        let fg: Color = match printer.focused {
            true => Color::Dark(BaseColor::Red),
            false => Color::Light(BaseColor::Black)
        };

        let style = ColorStyle::new(fg, Color::Dark(BaseColor::White));

        for (i, line) in self.text.lines().enumerate() {
            printer.with_color(style, 
                |printer| printer.print(( (MARGIN.0/2), i + (MARGIN.1/2)), &line.to_string()),
            )
        }

        let splash_y: usize = self.text.lines().count() + 1;
        printer.with_color(style, 
            |printer| printer.print((0, splash_y), &self.message),
        )

    }

    fn on_event(&mut self, e: Event) -> EventResult {
        /*
        if !self.selected {
            return EventResult::Ignored;
        }

        match e {
            Event::Key(Key::Up) => {
                self.selected = !self.selected;
            },
            Event::Key(Key::Down) => {
                self.selected = !self.selected;
            },
            _ => return EventResult::Consumed(None),
        }

        */
        EventResult::Ignored
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
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
