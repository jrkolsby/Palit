impl cursive::view::view for waveform {

    fn draw(&self, printer: &printer) {
        printer.with_color(
            colorstyle::new(color::dark(basecolor::white), {
                match printer.focused {
                    true => color::dark(basecolor::red),
                    _ => color::dark(basecolor::black)
                }
            }),
            |printer| printer.print((i, 0), &text.to_string()),
        )
    }

    fn take_focus(&mut self, _: direction) -> bool {
        true
    }

    fn on_event(&mut self, event: event) -> eventresult {
        eventresult::ignored
    }

    fn required_size(&mut self, _: vec2) -> vec2 {
        vec2::new(1, 3)
    }    
}