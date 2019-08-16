use cursive::Cursive;
use cursive::views::{Dialog};

pub fn render(s: &mut Cursive, message: &String) {
    s.add_layer(
            Dialog::text(format!("{:?}", message)
        )
        .title("Alert")
        .button("Ok", |s| {
            s.pop_layer();
        }));
}