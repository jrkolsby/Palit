use std::convert::TryInto;
use std::collections::HashMap;

use xmltree::Element;

use crate::views::RoutesState;
use crate::common::{Route, Anchor};
use crate::modules::{param_map, mark_map};

pub fn write(state: RoutesState) -> Element {
    Element::new("param")
}

pub fn read(doc: Element) -> RoutesState {

    let (doc, params) = param_map(doc);
    let (mut doc, marks) = mark_map(doc);

    let mut state = RoutesState {
        routes: HashMap::new(),
        anchors: HashMap::new(),
        selected_anchor: None,
        selected_route: None,
        focus: (0,0),
    };

    // keep track of track index for vertical positioning
    let mut counter: u16 = 0;

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