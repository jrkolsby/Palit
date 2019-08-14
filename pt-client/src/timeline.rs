use wavefile::WaveFile;
use itertools::Itertools;

const WAVEFORM_WIDTH: usize = 20;
const BRAILLE_HEIGHT: usize = 4;

// Cursive Functions
// pub fn step(&mut self) -> bool
// index.add_global_callback('l', |s| s.quit());
// pub fn print<S: Into<Vec2>>(&self, start: S, text: &str)

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
