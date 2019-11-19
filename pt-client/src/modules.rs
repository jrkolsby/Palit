use std::collections::HashMap;

use xmltree::Element;

use crate::common::{Param, Offset, DocID};

pub mod arpeggio;
pub mod hammond;
pub mod timeline;

const PALIT_ROOT: &str = "/usr/local/palit/";

pub fn param_map(mut doc: Element) -> (Element, HashMap<String, Param>) {
    let mut params: HashMap<String, Param> = HashMap::new();
    while let Some(param) = doc.take_child("param") {
        let key = param.attributes.get("name").unwrap();
        let val = param.attributes.get("value").unwrap();
        params.insert(key.to_string(), val.parse::<Param>().unwrap());
    }
    return (doc, params);
}

pub fn mark_map(mut doc: Element) -> (Element, HashMap<String, Offset>) {
    let mut marks: HashMap<String, Offset> = HashMap::new();
    while let Some(param) = doc.take_child("mark") {
        let key = param.attributes.get("name").unwrap();
        let val = param.attributes.get("value").unwrap();
        marks.insert(key.to_string(), val.parse::<Offset>().unwrap());
    }
    return (doc, marks);
}

/* 
    In the end, we need to take a document and return a list of views with
    ids, as well as set the project title and sample and bit rates
*/

pub fn read_document(path: String) -> HashMap<u16, Element> {
    HashMap::new()
}