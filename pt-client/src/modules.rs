use std::collections::HashMap;
use std::fs;

use xmltree::Element;

use crate::common::{Param, Offset};

pub mod timeline;
pub mod patch;

pub struct Document {
    pub title: String,
    pub sample_rate: u32,
    pub modules: HashMap<u16, Element>,
}

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

pub fn read_document(filename: String) -> Document {

    let doc_path: String = format!("{}{}", PALIT_ROOT, filename);
    let doc_str: String = fs::read_to_string(doc_path).unwrap();
    let mut doc: Element = Element::parse(doc_str.as_bytes()).unwrap();

    let mut result = Document {
        title: "Untitled".to_string(),
        sample_rate: 48000,
        modules: HashMap::new(),
    };

    if let Some(title) = doc.take_child("title") {
        result.title = title.text.unwrap().to_string();
    }

    if let Some(meta) = doc.take_child("meta") {
        if let Some(rate_str) = meta.attributes.get("samplerate") {
            result.sample_rate = rate_str.parse::<u32>().unwrap();
        }
    }

    if let Some(modules) = doc.take_child("modules") {
        for module in modules.children.iter() {
            if let Some(i) = module.attributes.get("id") {
                result.modules.insert(i.parse::<u16>().unwrap(), module.to_owned());
            } else {
                panic!("Module missing ID");
            }
        }
    } else {
        panic!("No modules defined!");
    }

    result
}