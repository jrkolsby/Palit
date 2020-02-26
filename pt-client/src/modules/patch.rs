use std::convert::TryInto;
use std::collections::HashMap;
use xmltree::Element;
use libcommon::{Route, Anchor, Param, param_map, mark_map};

use crate::views::PatchState;

pub fn write(state: PatchState) -> Element {
    eprintln!("WRITE {:?}", state);
    let mut root = Element::new("patch");

    let mut has_master: bool = false;

    for (id, route) in state.routes.iter() {
        if *id == 1 { has_master = true; }
        let mut route_el = Element::new("route");
        route_el.attributes.insert("id".to_string(), id.to_string());
        for anchor in route.patch.iter() {
            let mut anchor_el = if anchor.input { 
                Element::new("input") 
            } else {
                Element::new("output")
            };
            anchor_el.attributes.insert("module".to_string(), anchor.module_id.to_string());
            anchor_el.attributes.insert("index".to_string(), anchor.index.to_string());
            route_el.children.push(anchor_el);
        }
        root.children.push(route_el);
    }

    // Make sure there's always a master route
    if !has_master {
        let mut master_route = Element::new("route");
        master_route.attributes.insert("id".to_string(), "1".to_string());
        root.children.push(master_route);
    }

    root
}

pub fn read(mut doc: Element) -> PatchState {

    let (mut doc, params) = param_map(&mut doc);
    let (mut doc, marks) = mark_map(&mut doc);

    let mut state = PatchState {
        routes: HashMap::new(),
        anchors: HashMap::new(),
        selected_anchor: None,
        selected_route: None,
        focus: (0,0),
    };

    while let Some(mut route) = doc.take_child("route") {
        let r_id: &str = route.attributes.get("id").unwrap();
        let _r_id = r_id.parse::<u16>().unwrap();

        let mut current_route = Route {
            id: _r_id,
            patch: vec![]
        };

        while let Some(mut anchor) = route.take_child("input") {
            let a_id = anchor.attributes.get("index").unwrap();
            let _a_id = a_id.parse::<u16>().unwrap();
            let m_id = anchor.attributes.get("module").unwrap();
            let _m_id = m_id.parse::<u16>().unwrap();

            let anchor = Anchor {
                index: _a_id,
                module_id: _m_id,
                name: format!("In {}", a_id),
                input: true,
            };

            current_route.patch.push(anchor.clone());
            state.anchors.insert(anchor.index, anchor);
        }

        while let Some(mut anchor) = route.take_child("output") {
            let a_id = anchor.attributes.get("index").unwrap();
            let _a_id = a_id.parse::<u16>().unwrap();
            let m_id = anchor.attributes.get("module").unwrap();
            let _m_id = m_id.parse::<u16>().unwrap();

            let anchor = Anchor {
                index: _a_id,
                module_id: _m_id,
                name: format!("Out {}", a_id),
                input: false,
            };

            current_route.patch.push(anchor.clone());
            state.anchors.insert(anchor.index, anchor);
        }

        state.routes.insert(_r_id, current_route);
    }
    
    return state;
}