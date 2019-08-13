extern crate braille;
extern crate cursive;
extern crate musical_keyboard;
extern crate wavefile;
extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use cursive::Cursive;
use cursive::views::{Dialog, TextView, SelectView, DummyView, LinearLayout, EditView, Button};
use cursive::traits::{Identifiable, Boxable};

use std::collections::HashMap;



#[derive(Debug)]
struct Region<'a, 'b> {
    file: &'a str,
    sample: &'b Vec<Vec<i32>>,
    size: u32
}

struct Project<'a> {
    x: i32,
    name: String,
    regions: &'a HashMap<i32, String>
}

impl<'a> Project<'a> {
    fn save(&self) -> i32 { self.x }
}

mod daw_server {

}

mod client {

    use wavefile::WaveFile;
    use itertools::Itertools;

    const WAVEFORM_WIDTH: usize = 20;
    const BRAILLE_HEIGHT: usize = 4;

    // Cursive Functions
    // pub fn step(&mut self) -> bool
    // index.add_global_callback('l', |s| s.quit());
    // pub fn print<S: Into<Vec2>>(&self, start: S, text: &str)

    pub fn timeline() {}
    
    pub fn waveform(input: &str) -> String {
        let wav = match WaveFile::open(input) {
            Ok(f)  => f,
            Err(e) => panic!("{}",  e)
        };

        let chunk_size = wav.len() / WAVEFORM_WIDTH;
        let chunks = &wav.iter().chunks(chunk_size);

        let values = chunks.into_iter().map( |chunk| {
            let max = chunk.into_iter().map( |frame| {
                frame.iter().map(|sample| sample.abs()).max().unwrap()
            }).max().unwrap();
            max
        }).take(WAVEFORM_WIDTH).collect::<Vec<i32>>();

        let global_max = *values.iter().max().unwrap();
        let scale: f64 = BRAILLE_HEIGHT as f64 / global_max as f64;

        //println!("GLOBAL_MAX: {}", global_max);
        //println!("SCALE: {}", scale);

        let mut base: String = "".to_string();
        for (i, value) in values.iter().enumerate() {
            if i % 2 > 0 {
                continue;
            }
            //println!("{:?} , {:?}", *value, values[i+1]);
            let tick: (i32, i32) = (((*value as f64) * scale).round() as i32, 
                                    ((values[i+1] as f64) * scale).round() as i32);

            //println!("TICK: {}, {}", tick.0, tick.1);

            let unit = pair_to_char(tick);

            //println!("CHAR: {}", unit);

            base = format!("{}{}", base, unit);
        }
        base
    }

    pub fn pair_to_char(pair: (i32, i32)) -> char {

        // MAX IS 65,536
        let a: usize = (pair.0 >= 1) as usize;
        let b: usize = (pair.0 >= 2) as usize;
        let c: usize = (pair.0 >= 3) as usize;
        let d: usize = (pair.0 >= 4) as usize;
        let e: usize = (pair.1 >= 1) as usize;
        let f: usize = (pair.1 >= 2) as usize;
        let g: usize = (pair.1 >= 3) as usize;
        let h: usize = (pair.1 >= 4) as usize;

        braille::BRAILLE[d][h]
                        [c][g]
                        [b][f]
                        [a][e]
    }
}

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

fn alert(s: &mut Cursive, text: String) {
    s.add_layer(Dialog::text(format!("{:?}", text))
        .title("Alert")
        .button("Ok", |s| {
            s.pop_layer();
        }));
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
    alert(s, client::waveform("examples/test.wav"));
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

    index.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
            .title("Select a Project"));

    index.add_global_callback('q', |s| s.quit());
    index.add_global_callback('~', |s| s.toggle_debug_console());

    if let Err(_) = index.load_theme_file("examples/theme.toml") {
        let _ = index.load_theme_file("examples/theme.toml");
    }

    index.run();

    Ok(())
}
