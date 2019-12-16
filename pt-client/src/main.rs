extern crate libc;
extern crate termion;

use std::io::{Write, Stdout, stdout};
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;

use std::collections::VecDeque;

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};

// NOTE: These need to be here
mod views;
mod common;
mod components; 
mod modules;

use views::{Layer, Home, Timeline, Help, Title, Piano, Routes};
use modules::{read_document};

use common::{Action, Anchor};

fn render(mut stdout: RawTerminal<Stdout>, layers: &VecDeque<(u16, Box<Layer>)>) -> RawTerminal<Stdout> {
    /*
        LAYERS:
        4:   ---    <- End render here
        3:  -----
        2: -------  <- Start render here (!alpha)
        1:   ---
        0: -------  <- Home
    */
    // Determine bottom layer
    if layers.len() == 0 { return stdout; }
    let mut bottom = layers.len()-1;
    let target = bottom;
    for (id, layer) in (*layers).iter().rev() {
        if layer.alpha() { bottom -= 1 }
        else { break }
    };
    for i in bottom..(*layers).len() {
        stdout = layers[i].1.render(stdout, i == target);
    }
    stdout
}

fn ipc_action(mut ipc_in: &File) -> Vec<Action> {
    let mut buf: String = String::new();

    ipc_in.read_to_string(&mut buf);
    let mut ipc_iter = buf.split(" ");

    let mut events: Vec<Action> = Vec::new();

    while let Some(action_raw) = ipc_iter.next() {
        let argv: Vec<&str> = action_raw.split(":").collect();

        let action = match argv[0] {
            "ROUTE" => Action::Route,

            "TICK" => Action::Tick,

            "?" => Action::Noop,

            "1" => Action::Help,
            "2" => Action::Back,

            "PLAY" => Action::Play,
            "STOP" => Action::Stop,

            "M" => Action::SelectG,
            "R" => Action::SelectY,
            "V" => Action::SelectP,
            "I" => Action::SelectB,
            "SPC" => Action::SelectR,

            "NOTE_ON" => Action::NoteOn(argv[1].parse::<u16>().unwrap(), 
                                        argv[2].parse::<f32>().unwrap()),

            "NOTE_OFF" => Action::NoteOff(argv[1].parse::<u16>().unwrap()),

            "UP" => Action::Up,
            "DN" => Action::Down,
            "LT" => Action::Left,
            "RT" => Action::Right,

            "EXIT" => Action::Exit,

            "DESELECT" => Action::Deselect,

            _ => { Action::Noop },
        };

        match action {
            Action::Noop => {},
            a => { events.push(a); }
        };
    };

    events
}
fn add_layer(a: &mut VecDeque<(u16, Box<Layer>)>, b: Box<Layer>, id: u16) {
    a.push_back((id, b));
}

fn main() -> std::io::Result<()> {

    // Public action fifo /tmp/pt-client
    let mut ipc_in = OpenOptions::new()
        .custom_flags(libc::O_NONBLOCK)
	    .read(true)
	    .open("/tmp/pt-client").unwrap();

    // Blocked by pt-sound reader
    // If a process writes to stdout and nobody 
    // is around to read it, should it continue?
    println!("Waiting for pt-sound...");

    let mut ipc_sound = OpenOptions::new()
        .write(true)
        .open("/tmp/pt-sound").unwrap();

    // Configure raw_mode stdout
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Configure input polling array
    let in_src = CString::new("/tmp/pt-client").unwrap();
    let mut fds: Vec<libc::pollfd> = unsafe {vec![
        libc::pollfd { 
            fd: libc::open(in_src.as_ptr(), libc::O_RDONLY),
            events: libc::POLLIN,
            revents: 0,
        },
    ]};

    // Configure margins and sizes
    let size: (u16, u16) = terminal_size().unwrap();

    // Configure UI layers
    let mut layers: VecDeque<(u16, Box<Layer>)> = VecDeque::new();
    let mut routes_id: Option<u16> = None;

    add_layer(&mut layers, Box::new(Home::new(1, 1, size.0, size.1)), 0);

    // Hide cursor and clear screen
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    // Initial Render
    stdout = render(stdout, &layers);
    stdout.flush().unwrap();

    let mut events: Vec<Action> = Vec::new();

    'event: loop {

        unsafe{
            libc::poll(&mut fds[0] as *mut libc::pollfd, fds.len() as libc::nfds_t, 100);
        }

        // If anybody else closes the pipe, halt TODO: Throw error
        if fds[0].revents & libc::POLLHUP == libc::POLLHUP { break 'event; }
        let mut events: Vec<Action> = if fds[0].revents > 0 {
            ipc_action(&mut ipc_in)
        } else { continue; };

        while let Some(next) = events.pop() {

            // Target the top layer
            let num_views = layers.len();
            let (target_index, target_id) = if num_views > 0 {
                let index = num_views - 1;
                let id = match layers.get(index).unwrap() {
                    (id, _) => *id,
                    _ => 0
                };
                (index, id)
            } else {
                (0, 0)
            };

            // Execute toplevel actions, capture default from view
            let default: Action = match next {
                Action::Exit => { 
                    break 'event;
                },
                Action::Help => { 
                    add_layer(&mut layers, Box::new(Help::new(10, 10, 44, 15)), 0); 
                    Action::Noop
                },
                Action::Back => {
                    if let Some(current) = layers.pop_back() {
                        layers.push_front(current);
                    }
                    Action::Noop
                },
                a => {
                    let (_, target) = layers.get_mut(target_index).unwrap();
                    target.dispatch(a)
                }
            };

            // capture default action if returned from layer
            match default {
                Action::InputTitle => {
                    add_layer(&mut layers, Box::new(Title::new(23, 5, 36, 23)), 0);
                },
                Action::Play => {
                    ipc_sound.write(
                        format!("PLAY:{} ", target_id).as_bytes()).unwrap();
                }
                Action::Stop => {
                    ipc_sound.write(
                        format!("STOP:{} ", target_id).as_bytes()).unwrap();
                }
                Action::NoteOn(k, v) => {
                    ipc_sound.write(
                        format!("NOTE_ON_AT:{}:{}:{} ", target_id, k, v).as_bytes()).unwrap();
                },
                Action::NoteOff(k) => {
                    ipc_sound.write(
                        format!("NOTE_OFF_AT:{}:{} ", target_id, k).as_bytes()).unwrap();
                },
                a @ Action::Up | 
                a @ Action::Down => {
                    // If routes is the target view (top) remove it
                    let mut routes: Option<(u16, Box<Layer>)> = None;
                    if routes_id.is_some() && routes_id.unwrap() == target_id {
                        if let Some(top) = layers.pop_back() {
                            routes = Some(top);
                        }
                    };
                    // Slide layers over
                    match a {
                        Action::Up => {
                            if let Some(current) = layers.pop_front() {
                                layers.push_back(current);
                            }
                        },
                        Action::Down => {
                            if let Some(current) = layers.pop_back() {
                                layers.push_front(current);
                            }
                        },
                        _ => {}
                    }
                    // Make sure that routes view is not on top
                    // Restore routes view if it was the target...
                    if let Some(mut r) = routes { 
                        // ... and give it a new set of anchors
                        let action = layers[target_index-1].1.dispatch(Action::Route);
                        r.1.dispatch(action); 
                        layers.push_back(r);
                    }
                }, 
                /*
                Action::Pepper => {
                    add_layer(&mut layers, Box::new(Help::new(10, 10, 44, 15)), 0); 
                },
                Action::CreateProject(title) => {
                    ipc_sound.write(format!("NEW_PROJECT:{} ", title).as_bytes());
                    add_layer(&mut layers, Box::new(Timeline::new(1, 1, size.0, size.1, title)));
                },
                Action::Error(message) => {
                    layers.push(Box::new(Error::new(message))) ;
                }
                */
                Action::OpenProject(title) => {
                    ipc_sound.write(
                        format!("OPEN_PROJECT:{} ", title).as_bytes()).unwrap();
                    let doc = read_document(title);
                    for (id, el) in doc.modules.iter() {
                        match &el.name[..] {
                            "timeline" => add_layer(&mut layers, 
                                Box::new(Timeline::new(1, 1, size.0, size.1, (*el).to_owned())), *id),
                            "hammond" => add_layer(&mut layers,
                                Box::new(Piano::new(1,1,size.0,size.1, (*el).to_owned())), *id),
                            "patch" => { 
                                routes_id = Some(*id);
                                add_layer(&mut layers, Box::new(
                                    Routes::new(1,1,size.0,size.1, Some((*el).to_owned()))
                                ), *id) 
                            },
                            name => { eprintln!("unimplemented module {:?}", name)}
                        }
                    }
                },
                Action::ShowAnchors(anchors) => {
                    if let Some(r_id) = routes_id {
                        let mut routes_index: Option<usize> = None;

                        for (i, (id, layer)) in layers.iter_mut().enumerate() {
                            if *id == r_id {
                                routes_index = Some(i);
                            }
                        }

                        if let Some(j) = routes_index {
                            let (_, mut routes) = layers.remove(j).unwrap();

                            let anchors_fill = anchors.iter().map(|a| Anchor {
                                id: a.id,
                                module_id: target_id,
                                name: a.name.clone(),
                                x: a.x,
                                y: a.y,
                                input: a.input
                            }).collect();

                            match routes.dispatch(Action::ShowAnchors(anchors_fill)) {
                                Action::CountRoutes(num) => {
                                    add_layer(&mut layers, routes, r_id);
                                    for (_, l) in layers.iter_mut() {
                                        l.shift(num+1, 1);
                                    }
                                },
                                _ => { panic!("Patch failed to report number of routes"); }
                            }
                        }
                    } else {
                        routes_id = Some(1000);

                        add_layer(&mut layers, Box::new(
                            Routes::new(1,1,size.0,size.1, None)
                        ), 1000);
                    }
                },
                _ => {}
            };	
        }

        // Clears screen
        write!(stdout, "{}{}", color::Bg(color::Reset), clear::All).unwrap();
        stdout.flush().unwrap();

        // Renders layers
        stdout = render(stdout, &layers);
    }

    // CLEAN UP
    write!(stdout, "{}{}{}", 
        clear::All, 
        cursor::Goto(1,1), 
        cursor::Show).unwrap();

    Ok(())
}
