extern crate libc;
extern crate termion;
extern crate linux_raw_input_rs;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{BufReader, Write, Stdout, stdout, stdin};
use std::io::prelude::*;
use std::fs::{OpenOptions, File, read_to_string};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;

use linux_raw_input_rs::{InputReader, get_input_devices};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::input::EventType;

// ACTION FORMAT: CMD1:PARAM1:PARAM2 CMD2:PARAM1 ...

#[cfg(target_os = "macos")]
fn event_loop(mut ipc_client: File, mut ipc_sound: File) -> std::io::Result<()> {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        let key = c.unwrap();

        let client_buf: &str = match key.clone() {
            Key::Char('q') => "EXIT ",
            Key::Char('1') => "1 ",
            Key::Char('2') => "2 ",

            Key::Char('[') => "PLAY ",
            Key::Char(']') => "STOP ",

            Key::Char('m') => "M ",
            Key::Char('r') => "R ",
            Key::Char('v') => "V ",
            Key::Char('i') => "I ",
            Key::Char(' ') => "SPC ",

            Key::Char('\n') => "DESELECT ",
            Key::Char('\t') => "ROUTE ",

            Key::Char('a') => "NOTE_ON:24:0.4 ",
            Key::Char('w') => "NOTE_ON:25:0.4 ",
            Key::Char('s') => "NOTE_ON:26:0.4 ",
            Key::Char('e') => "NOTE_ON:27:0.4 ",
            Key::Char('d') => "NOTE_ON:28:0.4 ",
            Key::Char('f') => "NOTE_ON:29:0.4 ",
            Key::Char('t') => "NOTE_ON:30:0.4 ",
            Key::Char('g') => "NOTE_ON:31:0.4 ",
            Key::Char('y') => "NOTE_ON:32:0.4 ",
            Key::Char('h') => "NOTE_ON:33:0.4 ",
            Key::Char('u') => "NOTE_ON:34:0.4 ",
            Key::Char('j') => "NOTE_ON:35:0.4 ",
            Key::Char('k') => "NOTE_ON:36:0.4 ",
            Key::Char('o') => "NOTE_ON:37:0.4 ",
            Key::Char('l') => "NOTE_ON:38:0.4 ",
            Key::Char('p') => "NOTE_ON:39:0.4 ",
            Key::Char(';') => "NOTE_ON:40:0.4 ",

            Key::Up => "UP ",
            Key::Down => "DN ",
            Key::Left => "LT ",
            Key::Right => "RT ",

            _ => "",
        };

        let sound_buf: &str = match key.clone() {
            Key::Char('q') => "EXIT ",

            Key::Char('[') => "PLAY ",
            Key::Char(']') => "STOP ",

            Key::Char('a') => "NOTE_ON:24:0.4 ", // C1
            Key::Char('w') => "NOTE_ON:25:0.4 ",
            Key::Char('s') => "NOTE_ON:26:0.4 ",
            Key::Char('e') => "NOTE_ON:27:0.4 ",
            Key::Char('d') => "NOTE_ON:28:0.4 ",
            Key::Char('f') => "NOTE_ON:29:0.4 ",
            Key::Char('t') => "NOTE_ON:30:0.4 ",
            Key::Char('g') => "NOTE_ON:31:0.4 ",
            Key::Char('y') => "NOTE_ON:32:0.4 ",
            Key::Char('h') => "NOTE_ON:33:0.4 ",
            Key::Char('u') => "NOTE_ON:34:0.4 ",
            Key::Char('j') => "NOTE_ON:35:0.4 ",
            Key::Char('k') => "NOTE_ON:36:0.4 ",
            Key::Char('o') => "NOTE_ON:37:0.4 ",
            Key::Char('l') => "NOTE_ON:38:0.4 ",
            Key::Char('p') => "NOTE_ON:39:0.4 ",
            Key::Char(';') => "NOTE_ON:40:0.4 ",

            Key::Char('-') => "OCTAVE:0 ",
            Key::Char('+') => "OCTAVE:1 ",

            _ => "",
        };

        if client_buf.len() > 0 { ipc_client.write(client_buf.as_bytes()); }
        if sound_buf.len() > 0 { ipc_sound.write(sound_buf.as_bytes()); }

        if client_buf == "EXIT " { break; }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn event_loop(mut ipc_client: File, mut ipc_sound: File) -> std::io::Result<()> {

    // Configure keyboard input
    let keybd_path : String = get_input_devices().iter().nth(0).expect("Problem with iterator").to_string();
    println!("keyboard device: {}", keybd_path);
    let mut reader = InputReader::new(keybd_path.clone());

    // Keyboard Event Loop
    loop {

        // Block on keyboard input
        let input = reader.current_state();
        let event = (input.event_type(), input.get_key());

        let client_buf: &str = match event {
            /* (EventType::Release, _) => "GO", */
            (EventType::Push, k) => match k {
                Keys::KEY_Q => "EXIT ",
                Keys::KEY_1 => "1 ",
                Keys::KEY_2 => "2 ",

                Keys::KEY_LEFTBRACE => "PLAY ",
                Keys::KEY_RIGHTBRACE => "STOP ",

                Keys::KEY_M => "M ",
                Keys::KEY_R => "R ",
                Keys::KEY_V => "V ",
                Keys::KEY_I => "I ",
                Keys::KEY_SPACE => "SPC ",

                Keys::KEY_UP => "UP ",
                Keys::KEY_DOWN => "DN ",
                Keys::KEY_LEFT => "LT ",
                Keys::KEY_RIGHT => "RT ",

                _ => ""
            },
            (EventType::Release, k) => match k {
                Keys::KEY_M |
                Keys::KEY_R |
                Keys::KEY_V |
                Keys::KEY_I |
                Keys::KEY_SPACE => "DESELECT ",
            },
            (_, _) => ""
        };

        let sound_buf: &str = match event {
            (EventType::Push, k) => match k {
                Keys::KEY_A => "NOTE_ON:24:0.4 ",
                Keys::KEY_W => "NOTE_ON:25:0.4 ",
                Keys::KEY_S => "NOTE_ON:26:0.4 ",
                Keys::KEY_E => "NOTE_ON:27:0.4 ",
                Keys::KEY_D => "NOTE_ON:28:0.4 ",
                Keys::KEY_F => "NOTE_ON:29:0.4 ",
                Keys::KEY_T => "NOTE_ON:30:0.4 ",
                Keys::KEY_G => "NOTE_ON:31:0.4 ",
                Keys::KEY_Y => "NOTE_ON:32:0.4 ",
                Keys::KEY_H => "NOTE_ON:33:0.4 ",
                Keys::KEY_U => "NOTE_ON:34:0.4 ",
                Keys::KEY_J => "NOTE_ON:35:0.4 ",
                Keys::KEY_K => "NOTE_ON:36:0.4 ",
                Keys::KEY_O => "NOTE_ON:37:0.4 ",
                Keys::KEY_L => "NOTE_ON:38:0.4 ",
                Keys::KEY_P => "NOTE_ON:39:0.4 ",

                Keys::KEY_TAB => "OCTAVE:0 ",
                Keys::KEY_GRAVE => "OCTAVE:1 ",

                Keys::KEY_LEFTBRACE => "PLAY ",
                Keys::KEY_RIGHTBRACE => "STOP ",

                _ => ""
            },
            (EventType::Release, k) => match k {
                Keys::KEY_A => "NOTE_OFF:24 ",  
                Keys::KEY_W => "NOTE_OFF:25 ",  
                Keys::KEY_S => "NOTE_OFF:26 ",  
                Keys::KEY_E => "NOTE_OFF:27 ",  
                Keys::KEY_D => "NOTE_OFF:28 ",  
                Keys::KEY_F => "NOTE_OFF:29 ",  
                Keys::KEY_T => "NOTE_OFF:30 ",  
                Keys::KEY_G => "NOTE_OFF:31 ",  
                Keys::KEY_Y => "NOTE_OFF:32 ",  
                Keys::KEY_H => "NOTE_OFF:33 ",  
                Keys::KEY_U => "NOTE_OFF:34 ",  
                Keys::KEY_J => "NOTE_OFF:35 ",  
                Keys::KEY_K => "NOTE_OFF:36 ",  
                Keys::KEY_O => "NOTE_OFF:37 ",  
                Keys::KEY_L => "NOTE_OFF:38 ",  
                Keys::KEY_P => "NOTE_OFF:39 ",  
                _ => ""  
            },           
            (_, _) => ""
        };

        if client_buf.len() > 0 { ipc_client.write(client_buf.as_bytes()); }
        if sound_buf.len() > 0 { ipc_sound.write(sound_buf.as_bytes()); }
    };
}

fn main() -> std::io::Result<()> {

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

    println!("GO");
    event_loop(ipc_client, ipc_sound);

    Ok(())
}
