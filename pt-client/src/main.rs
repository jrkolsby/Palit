extern crate braille;
extern crate cursive;
extern crate musical_keyboard;
extern crate wavefile;
extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, SelectView, DummyView, LinearLayout, EditView, Button};
use cursive::traits::{Identifiable, Boxable};
use cursive::theme::{Theme, BorderStyle};

mod views;
mod core;
mod components;

use components::{alert, waveform, splash};


fn delete_name(s: &mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn ok(s: &mut Cursive, name: &str) {
    s.call_on_id("select", |view: &mut SelectView<String>| {
        view.add_item_str(name)
    });
    s.pop_layer();
}

fn add_name(s: &mut Cursive) {
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_id("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_id("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })

        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn on_submit(s: &mut Cursive, name: &String) {
    components::alert::render(s, &"OK!".to_string());
}

struct Project<'a> {
    x: i32,
    name: String,
    regions: &'a HashMap<i32, String>
}

impl<'a> Project<'a> {
    fn save(&self) -> i32 { self.x }
}

fn main() -> std::io::Result<()> {

    let secret = match File::open("examples/secret.txt") {
        Ok(f)  => f,
        Err(e) => panic!("{}",  e)
    };

    let mut buf_reader = BufReader::new(secret);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    let anonymous_proj: Project = Project {
        x: 0,
        name: "My Great Project".to_string(),
        regions: &HashMap::new()
    };

    //println!("{}", contents);
    //println!("{:?}", anonymous_proj.save());

    // Instantiate UI
    let mut index = Cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Delete Project", delete_name))
        .child(Button::new("New Project", add_name))
        .child(DummyView)
        .child(Button::new("Shutdown", Cursive::quit));

    /*
    index.add_layer(splash::Splash::new("Tell your friends!")
        .with_id("splash"));
       */

    index.add_layer(LinearLayout::vertical()
        .child(splash::Splash::new(""))
        .child(DummyView)
        .child(splash::Splash::new("It's Fun!"))
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Shutdown", Cursive::quit))
        );

    /*
    index.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
            .title("Select a Project"));

    */
    index.add_global_callback('q', |s| s.quit());
    index.add_global_callback('~', |s| s.toggle_debug_console());

    if let Err(_) = index.load_theme_file("src/style/theme.toml") {
        let _ = index.load_theme_file("src/style/theme.toml");
    }

/*
    index.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette: index.current_theme().palette
    });
    */

    index.run();

    Ok(())
}
