use cursive::Cursive;
use cursive::views::{Dialog};

pub fn alert(s: &mut Cursive, message: &str) {
    s.add_layer(
            Dialog::text(format!("{:?}", message.to_string())
        )
        .title("Alert")
        .button("Ok", |s| {
            s.pop_layer();
        }));
}