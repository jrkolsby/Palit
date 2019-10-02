extern crate libc;
extern crate linux_raw_input_rs;

use std::io::{BufReader, Write, Stdout, stdout, stdin};
use std::io::prelude::*;
use std::fs::{OpenOptions, read_to_string};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;

use linux_raw_input_rs::{InputReader, get_input_devices};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::input::EventType;

fn main() -> std::io::Result<()> {

    // Configure keyboard input
    let keybd_path : String = get_input_devices().iter().nth(0).expect("Problem with iterator").to_string();

    // Configure pt-client IPC
    println!("Waiting for pt-client...");

    // Blocked by pt-client reader
    let mut ipc_client = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-client").unwrap();

    println!("Waiting for pt-sound...");

    // Blocked by pt-client reader
    let mut ipc_sound = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-sound").unwrap();

    eprintln!("keyboard device: {}", keybd_path);
    let mut reader = InputReader::new(keybd_path.clone());

    // Keyboard Event Loop
    loop {

        // Block on keyboard input
        let input = reader.current_state();
        let event = (input.event_type(), input.get_key());

        let client_buf: &str = match event {
            /* (EventType::Release, _) => "GO", */
            (EventType::Push, k) => match k {
                Keys::KEY_Q => "EXIT",
                Keys::KEY_1 => "1",
                Keys::KEY_2 => "2",

                Keys::KEY_LEFTBRACE => "PLAY",
                Keys::KEY_RIGHTBRACE => "STOP",

                Keys::KEY_M => "M",
                Keys::KEY_R => "R",
                Keys::KEY_V => "V",
                Keys::KEY_I => "I",
                Keys::KEY_SPACE => "SPC",

                Keys::KEY_UP => "UP",
                Keys::KEY_DOWN => "DN",
                Keys::KEY_LEFT => "LT",
                Keys::KEY_RIGHT => "RT",
                _ => ""
            }
            (_, _) => ""
        };

        let sound_buf: &str = match event {
            (EventType::Push, k) => match k {
                Keys::KEY_A => "C1_ON",  
                Keys::KEY_W => "C1#_ON", 
                Keys::KEY_S => "D1_ON",  
                Keys::KEY_E => "D1#_ON", 
                Keys::KEY_D => "E1_ON",
                Keys::KEY_F => "F1_ON",  
                Keys::KEY_T => "F1#_ON", 
                Keys::KEY_G => "G1_ON",  
                Keys::KEY_Y => "G1#_ON", 
                Keys::KEY_H => "A1_ON",  
                Keys::KEY_U => "A1#_ON", 
                Keys::KEY_J => "B1_ON",  
                Keys::KEY_K => "C2_ON",  
                Keys::KEY_O => "C2#_ON", 
                Keys::KEY_L => "D2_ON",  
                Keys::KEY_P => "D2#_ON", 
                _ => ""
            },
            (EventType::Release, k) => match k {
                Keys::KEY_A => "C1_OFF",  
                Keys::KEY_W => "C1#_OFF", 
                Keys::KEY_S => "D1_OFF",  
                Keys::KEY_E => "D1#_OFF", 
                Keys::KEY_D => "E1_OFF",
                Keys::KEY_F => "F1_OFF",  
                Keys::KEY_T => "F1#_OFF", 
                Keys::KEY_G => "G1_OFF",  
                Keys::KEY_Y => "G1#_OFF", 
                Keys::KEY_H => "A1_OFF",  
                Keys::KEY_U => "A1#_OFF", 
                Keys::KEY_J => "B1_OFF",  
                Keys::KEY_K => "C2_OFF",  
                Keys::KEY_O => "C2#_OFF", 
                Keys::KEY_L => "D2_OFF",  
                Keys::KEY_P => "D2#_OFF", 
                _ => ""  
            },           
            (_, _) => ""
        };

        if client_buf.len() > 0 { ipc_client.write(client_buf.as_bytes()); }
        if sound_buf.len() > 0 { ipc_sound.write(sound_buf.as_bytes()); }
    };

    Ok(())
}
