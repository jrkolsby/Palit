use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Debug)]
struct Region<'a> {
    file: &'a str,
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

fn main() -> std::io::Result<()> {

    let file = File::open("foo.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

    let anonymous_proj: Project = Project {
        x: 0,
        name: "My Great Project".to_string(),
        regions: &HashMap::new()
    };

    println!("{}", contents);
    println!("{:?}", anonymous_proj.save());

    Ok(())
}
