extern crate libcommon;
extern crate libc;
extern crate termion;

use std::io::{Write, Stdout, stdout, BufWriter};
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::CString;
use std::os::unix::io::FromRawFd;
use std::ops::DerefMut;
use std::collections::VecDeque;
use std::{thread, time};
use xmltree::Element;
use termion::{clear, color, cursor, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};
use libcommon::{Action, Module, Anchor};

// NOTE: These need to be here
mod views;
mod common;
mod components; 
mod modules;

use views::{Layer, 
    Home, 
    Timeline, 
    Help, 
    Title, 
    Piano, 
    Routes, 
    Keyboard, 
    Arpeggio,
    Modules,
    Project,
    Plugin,
};
use modules::{read_document, Document};

use common::{Screen, MARGIN_D0, MARGIN_D1, MARGIN_D2};

const DEFAULT_ROUTE_ID: u16 = 29200;
const DEFAULT_HOME_ID: u16 = 29201;
const DEFAULT_HELP_ID: u16 = 29202;
const DEFAULT_MODULES_ID: u16 = 29203;
const DEFAULT_PROJECT_ID: u16 = 29204;

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
        match action_raw.parse::<Action>() {
            Ok(Action::Noop) => (),
            Ok(a) => events.push(a),
            Err(a) => (),
        };
    };

    events
}
fn add_layer(a: &mut VecDeque<(u16, Box<Layer>)>, b: Box<Layer>, id: u16) {
    a.push_back((id, b)); // End of layers is front of the screen
}

fn add_module(
    a: &mut VecDeque<(u16, Box<Layer>)>, 
    name: &str, 
    id: u16, 
    size: (u16, u16), 
    el: Element) {
    match name {
        "timeline" => add_layer(a, 
            Box::new(Timeline::new(1, 1, size.0, size.1, (el).to_owned())), id),
        "hammond" => add_layer(a,
            Box::new(Piano::new(5,5,size.0,size.1, (el).to_owned())), id),
        "keyboard" => add_layer(a,
            Box::new(Keyboard::new(1, 1, size.0, size.1, (el).to_owned())), id),
        "arpeggio" => add_layer(a,
            Box::new(Arpeggio::new(1, 1, size.0, size.1, (el).to_owned())), id),
        "patch" => { 
            let mut routes_index: Option<usize> = None;
            for (i, (_id, layer)) in a.iter_mut().enumerate() {
                if *_id == DEFAULT_ROUTE_ID {
                    routes_index = Some(i);
                    break;
                }
            }
            if routes_index.is_none() {
                add_layer(a, Box::new(
                    Routes::new(
                        MARGIN_D0.0,
                        MARGIN_D0.1,
                        size.0 - (MARGIN_D0.0 * 2),
                        size.1 - (MARGIN_D0.1 * 2), 
                    Some((el).to_owned()))
                ), DEFAULT_ROUTE_ID);
            }
        },
        "plugin" => add_layer(a,
            Box::new(Plugin::new(1, 1, size.0, size.1, (el).to_owned())), id),
        name => { eprintln!("unimplemented module {:?}", name)}
    }
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
        BufWriter::with_capacity(200_000, File::from_raw_fd(1)).into_raw_mode().unwrap()
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
    let mut document: Option<Document> = None;

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
                Action::Instrument => {
                    add_layer(&mut layers, Box::new(Modules::new(
                        MARGIN_D1.0,
                        MARGIN_D1.1, 
                        size.0 - (MARGIN_D1.0 * 2), 
                        size.1 - (MARGIN_D1.1 * 2),
                    )), DEFAULT_MODULES_ID); 
                    Action::Noop
                },
                Action::At(n_id, action) => {
                    let mut default = Action::Noop;
                    for (id, layer) in layers.iter_mut() {
                        if *id == n_id {
                            default = layer.dispatch(*action.to_owned())
                        }
                    }
                    default
                },
                a => {
                    let (_, target) = layers.get_mut(target_index).unwrap();
                    target.dispatch(a)
                }
            };

            // capture default action if returned from layer
            match default {
                Action::Cancel => { layers.pop_back(); },
                Action::Back => {
                    if let Some(current) = layers.pop_back() {
                        layers.push_front(current);
                    }
                },
                Action::InputTitle => {
                    add_layer(&mut layers, Box::new(Title::new(23, 5, 36, 23)), 0);
                },
                a @ Action::Up | 
                a @ Action::Down => {
                    // Make sure to pin {home|route|...|route?}
                    // Remove home and routes
                    let mut routes_i: Option<usize> = None;
                    let mut home_i: Option<usize> = None;
                    let mut pin_routes: bool = false;

                    for (i, (id, _)) in layers.iter_mut().enumerate() {
                        if *id == DEFAULT_ROUTE_ID {
                            routes_i = Some(i);
                            if target_id == DEFAULT_ROUTE_ID {
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
                                        index: a.index,
                                        module_id: *t_id,
                                        name: a.name.clone(),
                                        input: a.input
                                    }).collect();
                                    routes.1.dispatch(Action::ShowAnchors(anchors_fill)); 
                                    layers.push_back(routes)
                                },
                                _ => {
                                    routes.1.dispatch(Action::ShowAnchors(vec![])); 
                                    layers.push_back(routes)
                                }
                            }
                        } else {
                            layers.push_front(routes);
                        }
                    }
                }, 
                Action::OpenProject(title) => {
                    ipc_sound.write(format!("OPEN_PROJECT:{} ", title).as_bytes()).unwrap();
                    let mut doc = read_document(title);
                    ipc_sound.write(format!("SAMPLE_RATE:{} ", doc.sample_rate).as_bytes()).unwrap();
                    for (id, el) in doc.modules.iter() {
                        add_module(&mut layers, &el.name, *id, size, el.to_owned());
                    }
                    document = Some(doc.clone());
                },
                // SHOW PROJECT
                Action::Left => {
                    let mut project_view = Project::new(
                        MARGIN_D2.0,
                        MARGIN_D2.1, 
                        size.0 - (MARGIN_D2.0 * 2), 
                        size.1 - (MARGIN_D2.1 * 2),
                    );
                    if let Some(doc) = document {
                        let modules: Vec<Module> = doc.modules.iter().map(|(id, el)| Module {
                            id: id.clone(),
                            name: el.name.clone(),
                        }).collect();
                        project_view.dispatch(Action::ShowProject( doc.title.clone(), modules));
                        add_layer(&mut layers, Box::new(project_view), DEFAULT_PROJECT_ID); 
                        document = Some(doc);
                    }
                },
                Action::ShowAnchors(anchors) => {
                    let mut routes_index: Option<usize> = None;
                    for (i, (id, layer)) in layers.iter_mut().enumerate() {
                        if *id == DEFAULT_ROUTE_ID {
                            routes_index = Some(i);
                        }
                    }

                    if routes_index.is_none() {
                        add_layer(&mut layers, Box::new(
                            Routes::new(
                                MARGIN_D0.0,
                                MARGIN_D0.1,
                                size.0 - (MARGIN_D0.0 * 2),
                                size.1 - (MARGIN_D0.1 * 2), 
                                None
                            )
                        ), DEFAULT_ROUTE_ID);
                        routes_index = Some(layers.len()-1);
                    }

                    let (_, mut route_view) = layers.remove(routes_index.unwrap()).unwrap();

                    let (mod_id, _) = layers.get(layers.len()-1).unwrap();

                    let anchors_fill = anchors.iter().map(|a| Anchor {
                        index: a.index,
                        module_id: *mod_id,
                        name: a.name.clone(),
                        input: a.input
                    }).collect();
                    // Restore route view
                    route_view.dispatch(Action::ShowAnchors(anchors_fill));
                    add_layer(&mut layers, route_view, DEFAULT_ROUTE_ID);
                },
                Action::AddModule(0, name) => {
                    let mut new_id = match &name[..] {
                        "keyboard" => 104,
                        // Get next sequential ID
                        _ => layers.iter().fold(0, |max, (id,_)| 
                            if *id > max { *id } else { max }
                        ) + 1
                    };
                    ipc_sound.write(format!("ADD_MODULE:{}:{} ", new_id, name).as_bytes()).unwrap();
                    // Make empty element with tag and id
                    let mut new_el = Element::new(&name);
                    new_el.attributes.insert("id".to_string(), new_id.to_string());
                    add_module(&mut layers, &name, new_id, size, new_el.clone());
                    if let Some(mut doc) = document {
                        doc.modules.insert(new_id, new_el);
                        document = Some(doc);
                    }
                    // Make sure modules view is still in front so it can Cancel
                    layers.swap(layers.len()-1, layers.len()-2);
                    events.push(Action::Back);
                },
                Action::DelModule(id) => {
                    ipc_sound.write(format!("DEL_MODULE:{} ", id).as_bytes()).unwrap();
                    layers.retain(|(i, _)| *i != id);
                    if let Some(mut doc) = document {
                        doc.modules.retain(|i, _| *i != id);
                        document = Some(doc);
                    }
                    let mut routes_index: Option<usize> = None;
                    for (i, (l_id, _)) in layers.iter_mut().enumerate() {
                        if *l_id == DEFAULT_ROUTE_ID {
                            routes_index = Some(i);
                        }
                    }
                    if let Some(r_id) = routes_index {
                        layers[r_id].1.dispatch(Action::DelModule(id));
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
                a @ Action::DelRoute(_) |
                a @ Action::AddRoute(_) |
                a @ Action::PatchIn(_, _, _) |
                a @ Action::PatchOut(_, _, _) |
                a @ Action::DelPatch(_, _, _) => {
                    ipc_sound.write(a.to_string().as_bytes()).unwrap();
                },
                Action::Noop => {},
                direct_action => {
                    ipc_sound.write(Action::At(
                        target_id, 
                        Box::new(direct_action)
                    ).to_string().as_bytes()).unwrap();
                }
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
