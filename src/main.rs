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

use itertools::Itertools;

use wavefile::WaveFile;

const WAVEFORM_WIDTH: usize = 20;
const BRAILLE_HEIGHT: usize = 4;

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

mod render_audio {

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

    let wav = match WaveFile::open("examples/test.wav") {
        Ok(f)  => f,
        Err(e) => panic!("{}",  e)
    };

    alert(s, format!("{} Hz, {} channel(s), {} total samples", wav.sample_rate(), wav.channels(), wav.len()));

    let chunk_size = wav.len() / WAVEFORM_WIDTH;
    let chunks = &wav.iter().chunks(chunk_size);

    let values = chunks.into_iter().map( |chunk| {
        let max = chunk.into_iter().map( |frame| {
            frame.iter().map(|sample| sample.abs()).max().unwrap()
        }).max().unwrap();
        max
    }).take(WAVEFORM_WIDTH).collect::<Vec<i32>>();

    let global_max = *values.iter().max().unwrap();
    let mid        = BRAILLE_HEIGHT / 2;
    let scale      = mid as f32 / global_max as f32;

    alert(s, format!("{:?}", global_max));

    let waveform = {
        let mut base: String = ">".to_string();
        for (i, value) in values.iter().enumerate() {
            if i % 2 > 0 {
                continue;
            }
            let tick: (i32, i32) = (*value * scale as i32, 
                                    values[i+1] * scale as i32);
            base = format!("{:?}{:?}", base, 
                render_audio::pair_to_char(tick).to_string());
        }
        base
    };

    alert(s, format!("{:?}", waveform))
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

    println!("{}", contents);
    println!("{:?}", anonymous_proj.save());

    // Instantiate UI
    let mut siv = Cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Delete Project", delete_name))
        .child(Button::new("New Project", add_name))
        .child(DummyView)
        .child(Button::new("Shutdown", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
            .title("Select a profile"));

    siv.run();

    siv.add_global_callback('q', |s| s.quit());

    Ok(())
}
