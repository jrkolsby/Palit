extern crate libc;
extern crate termion;

use std::io::{Write, Stdout, stdout, BufWriter};
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;
use std::os::unix::io::FromRawFd;
use std::ops::DerefMut;
use std::{thread, time};

use std::collections::VecDeque;

use termion::{clear, color, cursor, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};

// NOTE: These need to be here
mod views;
mod common;
mod components; 
mod modules;

use views::{Layer, Home, Timeline, Help, Title, Piano, Routes, Keyboard};
use modules::{read_document};

use common::{Screen, Action, Anchor, MARGIN_D0, MARGIN_D1};

const DEFAULT_ROUTE_ID: u16 = 29200;
const DEFAULT_HOME_ID: u16 = 29201;
const DEFAULT_HELP_ID: u16 = 29202;

fn render(stdout: &mut Screen, layers: &VecDeque<(u16, Box<Layer>)>) {
    /*
        LAYERS:
        4:   ---    <- End render here
        3:  -----
        2: -------  <- Start render here (!alpha)
        1:   ---
        0: -------  <- Home
    */
    // Determine bottom layer
    if layers.len() == 0 { return; }
    let mut bottom = layers.len()-1;
    let target = bottom;
    for (id, layer) in (*layers).iter().rev() {
        if layer.alpha() { bottom -= 1 }
        else { break }
    };
    for i in bottom..(*layers).len() {
        layers[i].1.render(stdout, i == target);
    }
}

fn ipc_action(mut ipc_in: &File) -> Vec<Action> {
    let mut buf: String = String::new();

    // FIXME: Err but still writes to buf?
    match ipc_in.read_to_string(&mut buf) {
        _ => {}
    };
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

    // Allocate 8MB buffer in raw mode
    let mut out = unsafe {
        BufWriter::with_capacity(20_000, File::from_raw_fd(1)).into_raw_mode().unwrap()
    };

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

    add_layer(&mut layers, Box::new(
        Home::new(1, 1, size.0, size.1)
    ), DEFAULT_HOME_ID);

    // Initial Render
    write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
    render(&mut out, &layers);
    out.deref_mut().flush().unwrap();

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
                    add_layer(&mut layers, Box::new(Help::new(
                        MARGIN_D1.0,
                        MARGIN_D1.1, 
                        size.0 - (MARGIN_D1.0 * 2), 
                        size.1 - (MARGIN_D1.1 * 2),
                    )), DEFAULT_HELP_ID); 
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
                    ipc_sound.write( format!("PLAY:{} ", target_id).as_bytes()).unwrap();
                },
                Action::Stop => {
                    ipc_sound.write( format!("STOP:{} ", target_id).as_bytes()).unwrap();
                },
                Action::NoteOn(key, vel) => {
                    ipc_sound.write( format!("NOTE_ON_AT:{}:{}:{} ", 
                        target_id, key, vel).as_bytes()).unwrap();
                },
                Action::NoteOff(key) => {
                    ipc_sound.write( format!("NOTE_OFF_AT:{}:{} ", 
                        target_id, key).as_bytes()).unwrap();
                },
                Action::PatchOut(module_id, anchor_id, route_id) => {
                    ipc_sound.write( format!("PATCH_OUT:{}:{}:{} ", 
                        module_id, anchor_id, route_id).as_bytes()).unwrap();
                },
                Action::PatchIn(module_id, anchor_id, route_id) => {
                    ipc_sound.write( format!("PATCH_IN:{}:{}:{} ", 
                        module_id, anchor_id, route_id).as_bytes()).unwrap();
                },
                Action::DelPatch(module_id, anchor_id) => {
                    ipc_sound.write( format!("DEL_PATCH:{}:{} ", 
                        module_id, anchor_id).as_bytes()).unwrap();
                },
                Action::DelRoute(route_id) => {
                    ipc_sound.write( format!("DEL_ROUTE:{} ", 
                        route_id).as_bytes()).unwrap();
                },
                Action::AddRoute(route_id) => {
                    ipc_sound.write( format!("ADD_ROUTE:{} ", 
                        route_id).as_bytes()).unwrap();
                }
                a @ Action::Up | 
                a @ Action::Down => {
                    // Make sure to pin {home|route|...|route?}
                    // Remove home and routes
                    let mut routes_i: Option<usize> = None;
                    let mut home_i: Option<usize> = None;
                    let mut pin_routes: bool = false;

                    for (i, (id, _)) in layers.iter_mut().enumerate() {
                        if routes_id.is_some() && routes_id.unwrap() == *id {
                            routes_i = Some(i);
                            if routes_id.unwrap() == target_id {
                                pin_routes = true;
                            }
                        }
                        if *id == DEFAULT_HOME_ID {
                            home_i = Some(i)
                        }
                    }

                    // Remove home and routes
                    let (routes_view, home_view) = {
                        let h_i = home_i.unwrap();
                        if routes_i.is_some() {
                            let r_i = routes_i.unwrap();

                            if r_i > h_i {
                                let r = layers.remove(r_i);
                                let h = layers.remove(h_i);
                                (r,h)
                            } else {
                                let h = layers.remove(h_i);
                                let r = layers.remove(r_i);
                                (r,h)
                            }
                        } else {
                            (None, layers.remove(h_i))
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
                    if let Some(view) = home_view {
                        layers.push_front(view);
                    }
                    if let Some(mut routes) = routes_view {
                        if pin_routes {
                            let (t_id, target) = layers.get_mut(layers.len() - 1).unwrap();
                            match target.dispatch(Action::Route) {
                                Action::ShowAnchors(a) => {
                                    let anchors_fill = a.iter().map(|a| Anchor {
                                        id: a.id,
                                        module_id: *t_id,
                                        name: a.name.clone(),
                                        input: a.input
                                    }).collect();
                                    routes.1.dispatch(Action::ShowAnchors(anchors_fill)); 
                                    layers.push_back(routes)
                                },
                                _ => {}
                            }
                        } else {
                            layers.push_front(routes);
                        }
                    }
                }, 
                Action::OpenProject(title) => {
                    ipc_sound.write(
                        format!("OPEN_PROJECT:{} ", title).as_bytes()).unwrap();
                    let doc = read_document(title);
                    for (id, el) in doc.modules.iter() {
                        match &el.name[..] {
                            "timeline" => add_layer(&mut layers, 
                                Box::new(Timeline::new(1, 1, size.0, size.1, (*el).to_owned())), *id),
                            "hammond" => add_layer(&mut layers,
                                Box::new(Piano::new(5,5,size.0,size.1, (*el).to_owned())), *id),
                            "keyboard" => add_layer(&mut layers,
                                Box::new(Keyboard::new(1, 1, size.0, size.1, (*el).to_owned())), *id),
                            "patch" => { 
                                if let Some(r_id) = routes_id {
                                    let mut routes_index: Option<usize> = None;
                                    for (i, (_id, layer)) in layers.iter_mut().enumerate() {
                                        if *_id == r_id {
                                            routes_index = Some(i);
                                            break;
                                        }
                                    }
                                    if let Some(j) = routes_index {
                                        layers.remove(j);
                                    }
                                }
                                routes_id = Some(*id);
                                add_layer(&mut layers, Box::new(
                                    Routes::new(
                                        MARGIN_D0.0,
                                        MARGIN_D0.1,
                                        size.0 - (MARGIN_D0.0 * 2),
                                        size.1 - (MARGIN_D0.1 * 2), 
                                    Some((*el).to_owned()))
                                ), *id);
                            },
                            name => { eprintln!("unimplemented module {:?}", name)}
                        }
                    }
                },
                Action::ShowAnchors(anchors) => {
                    if routes_id.is_none() {
                        routes_id = Some(DEFAULT_ROUTE_ID);
                        add_layer(&mut layers, Box::new(
                            Routes::new(
                                MARGIN_D0.0,
                                MARGIN_D0.1,
                                size.0 - (MARGIN_D0.0 * 2),
                                size.1 - (MARGIN_D0.1 * 2), 
                                None
                            )
                        ), DEFAULT_ROUTE_ID);
                    }
                    let r_id = routes_id.unwrap();

                    let mut routes_index: Option<usize> = None;

                    for (i, (id, layer)) in layers.iter_mut().enumerate() {
                        if *id == r_id {
                            routes_index = Some(i);
                        }
                    }

                    if let Some(j) = routes_index {
                        let (_, mut route_view) = layers.remove(j).unwrap();

                        let (mod_id, _) = layers.get(layers.len()-1).unwrap();

                        let anchors_fill = anchors.iter().map(|a| Anchor {
                            id: a.id,
                            module_id: *mod_id,
                            name: a.name.clone(),
                            input: a.input
                        }).collect();

                        // Restore route view
                        route_view.dispatch(Action::ShowAnchors(anchors_fill));
                        add_layer(&mut layers, route_view, r_id);
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
                _ => {}
            };	
        }

        thread::sleep(time::Duration::from_millis(10));

        // Render layers 
        write!(out, "{}{}", color::Bg(color::Reset), clear::All).unwrap();
        render(&mut out, &layers);

        // Flush buffer
        // HACK ALERT! Without this sleep we experience flashing on render
        thread::sleep(time::Duration::from_millis(10));
        out.deref_mut().flush().unwrap();
        out.flush().unwrap();
    }

    // CLEAN UP
    write!(out, "{}{}{}{}{}", 
        clear::All, 
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        cursor::Goto(1,1), 
        cursor::Show).unwrap();
    out.deref_mut().flush().unwrap();

    Ok(())
}
