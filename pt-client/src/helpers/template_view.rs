impl cursive::view::View for Waveform {

    fn draw(&self, printer: &Printer) {
        printer.with_color(
            ColorStyle::new(Color::Dark(BaseColor::White), {
                match printer.focused {
                    true => Color::Dark(BaseColor::Red),
                    _ => Color::Dark(BaseColor::Black)
                }
            }),
            |printer| printer.print((i, 0), &text.to_string()),
        )
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(1, 3)
    }    
}