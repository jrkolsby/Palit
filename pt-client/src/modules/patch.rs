use std::convert::TryInto;
use std::collections::HashMap;
use xmltree::Element;
use libcommon::{Route, Anchor, Param, param_map, mark_map};

use crate::views::PatchState;

pub fn write(state: PatchState) -> Element {
    let mut root = Element::new("patch");

    let mut master_route = Element::new("route");
    master_route.attributes.insert("id".to_string(), "1".to_string());
    root.children.push(master_route);

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