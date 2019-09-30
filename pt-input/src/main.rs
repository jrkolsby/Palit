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
    let mut ipc_out = OpenOptions::new()
	.write(true)
	.open("/tmp/pt-client").unwrap();

    eprintln!("keyboard device: {}", keybd_path);
    let mut reader = InputReader::new(keybd_path.clone());

    // Keyboard Event Loop
    loop {

        // Block on keyboard input
        let input = reader.current_state();
        let event = (input.event_type(), input.get_key());
        match event {
            (EventType::Release, _) => { /* ipc_out.write(b"GO"); */ },
            (EventType::Push, k) => match k {
                Keys::KEY_Q => { ipc_out.write(b"EXIT"); break; },
                Keys::KEY_1 => { ipc_out.write(b"1"); println!("1"); },
                Keys::KEY_2 => { ipc_out.write(b"2"); println!("2"); },

                Keys::KEY_LEFTBRACE => { ipc_out.write(b"PLAY"); println!("["); },
                Keys::KEY_RIGHTBRACE => { ipc_out.write(b"STOP"); println!("]"); },

                Keys::KEY_M => { ipc_out.write(b"M"); println!("M"); },
                Keys::KEY_R => { ipc_out.write(b"R"); println!("R"); },
                Keys::KEY_V => { ipc_out.write(b"V"); println!("V"); },
                Keys::KEY_I => { ipc_out.write(b"I"); println!("I"); },
                Keys::KEY_SPACE => { ipc_out.write(b"SPC"); println!(" "); },

                Keys::KEY_A => { ipc_out.write(b"A"); println!("A"); },
                Keys::KEY_S => { ipc_out.write(b"S"); println!("S"); },
                Keys::KEY_D => { ipc_out.write(b"D"); println!("D"); },
                Keys::KEY_F => { ipc_out.write(b"F"); println!("F"); },
                Keys::KEY_G => { ipc_out.write(b"G"); println!("G"); },
                Keys::KEY_H => { ipc_out.write(b"H"); println!("H"); },
                Keys::KEY_J => { ipc_out.write(b"J"); println!("J"); },
                Keys::KEY_K => { ipc_out.write(b"K"); println!("K"); },
                Keys::KEY_L => { ipc_out.write(b"L"); println!("L"); },

                Keys::KEY_UP => { ipc_out.write(b"UP"); println!("UP"); },
                Keys::KEY_DOWN => { ipc_out.write(b"DN"); println!("DN"); },
                Keys::KEY_LEFT => { ipc_out.write(b"LT"); println!("LT"); },
                Keys::KEY_RIGHT => { ipc_out.write(b"RT"); println!("RT"); }
                _ => {}
            }
            (_, _) => {}
        };
    }

    Ok(())
}
