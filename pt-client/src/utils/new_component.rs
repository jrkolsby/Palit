use cursive::{Printer, Vec2};
use cursive::theme::{Color, BaseColor, ColorStyle};
use cursive::event::{Event, EventResult};
use cursive::direction::Direction;

pub struct ColorButton {
    color: Color,
    text: String,
    primary: bool,
}

impl ColorButton {
    pub fn new(color: Color, text: String, primary: bool) -> Self {
        ColorButton {
            color: color,
            text: text,
            primary: primary,
        }
    }
}

impl cursive::view::View for ColorButton {

    fn draw(&self, printer: &Printer) {

        let fg = match printer.focused {
            true => Color::Dark(BaseColor::Black),
            false => Color::Light(BaseColor::White),
        };

        let bg = match printer.focused {
            true => self.color,
            false => Color::Dark(BaseColor::Black)
        };

        printer.with_color(
            ColorStyle::new(fg, bg), |printer| {
                printer.print((0, 0), " ");
                printer.print((1, 0), &self.text.to_string());
                printer.print((self.text.len()+1, 0), " ");
            }
        );
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(10,10)
    }    
}