use cursive::{Cursive, Printer};
use cursive::theme::{BaseColor, Color, ColorStyle, Effect};
use cursive::vec::Vec2;

pub struct Splash {
    message: String
}

const LOGO: &str = "
d8888b.  .d8b.  db      d888888b d888888b
88  `8D d8' `8b 88      `~~88~'  `~~88~~'
88oodD' 88ooo88 88         88       88   
88~~~   88~~~88 88         88       88   
88      88   88 88booo.   .88.      88   
YP      YP   YP Y88888P Y888888P    YP   
";

impl Splash {
    pub fn new(message: &str) -> Self {
        Splash {
            message: message.to_string()
        }
    }
}

impl cursive::view::View for Splash {
    fn draw(&self, printer: &Printer) {
        let style = ColorStyle::new(
            Color::Dark(BaseColor::White), 
            Color::Dark(BaseColor::Black));

        for (i, line) in LOGO.lines().enumerate() {
            printer.with_color(style, 
                |printer| printer.print((0,i), &line.to_string()),
            )
            //printer.print((0,i), &line.to_string());
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let width: usize = LOGO.lines().nth(1).unwrap().len();
        let height: usize = LOGO.lines().count();
        Vec2::new(width, height)
    }
}
