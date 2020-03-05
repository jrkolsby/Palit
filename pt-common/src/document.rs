use std::collections::HashMap;
use std::fs;
use xmltree::{Element, EmitterConfig};
use crate::{Note, Key, Volume, Param, Offset};

#[derive(Clone, Debug)]
pub struct Document {
    pub title: String,
    pub src: String,
    pub sample_rate: u32,
    pub modules: Vec<(u16, Element)>,
}

pub const PALIT_ROOT: &str = "./";

pub fn param_map(doc: &mut Element) -> (&mut Element, HashMap<String, Param>) {
    let mut params: HashMap<String, Param> = HashMap::new();
    while let Some(param) = doc.take_child("param") {
        let key = param.attributes.get("name").unwrap();
        let val = param.attributes.get("value").unwrap();
        params.insert(key.to_string(), val.parse::<Param>().unwrap());
    }
    return (doc, params);
}

pub fn mark_map(doc: &mut Element) -> (&mut Element, HashMap<String, Offset>) {
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
    let mut patch: Option<(u16, Element)> = None;

    let mut result = Document {
        src: filename,
        title: "Untitled".to_string(),
        sample_rate: 48000,
        modules: vec![],
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
                // Make sure patch is the last module in the result
                if module.name == "patch" {
                    patch = Some((i.parse::<u16>().unwrap(), module.to_owned()));
                    continue;
                }
                result.modules.push((i.parse::<u16>().unwrap(), module.to_owned()));
            } else {
                panic!("Module missing ID");
            }
        }
    } else {
        panic!("No modules defined!");
    }

    match patch {
        Some(p) => { result.modules.push(p); },
        None => {} // No need to panic, sound will add a master route
    }

    result
}

pub fn write_document(doc: &mut Document) {
    let mut root = Element::new("project");

    // Metadata declaration (sample rate, bitrate, project-wide)
    let mut meta = Element::new("meta");
    meta.attributes.insert("samplerate".to_string(), doc.sample_rate.to_string());
    root.children.push(meta);

    // Title declaration
    let mut title = Element::new("title");
    title.text = Some(doc.title.clone());
    root.children.push(title);

    // Modules declaration
    let mut modules = Element::new("modules");
    for (id, module_el) in doc.modules.iter_mut() { 
        module_el.attributes.insert("id".to_string(), id.to_string());
        modules.children.push(module_el.to_owned());
    } 
    root.children.push(modules);

    let doc_path: String = format!("{}{}.xml", PALIT_ROOT, doc.src);
    root.write_with_config(
        fs::File::create(doc_path).unwrap(), 
        EmitterConfig::new()
            .line_separator("\r\n")
            .perform_indent(true)
            .normalize_empty_elements(true));
}

pub fn note_list(doc: &mut Element, r_id: u16) -> (&mut Element, Vec<Note>) {
    let mut notes: Vec<Note> = vec![];
    while let Some(note) = doc.take_child("note") {
        notes.push(Note {
            id: note.attributes.get("id").unwrap().parse::<u16>().unwrap(),
            r_id,
            note: note.attributes.get("key").unwrap().parse::<Key>().unwrap(),
            t_in: note.attributes.get("t_in").unwrap().parse::<Offset>().unwrap(),
            t_out: note.attributes.get("t_out").unwrap().parse::<Offset>().unwrap(),
            vel: note.attributes.get("vel").unwrap().parse::<Volume>().unwrap(),
        });
    }
    return (doc, notes);
}

pub fn param_add<T>(el: &mut Element, value: T, name: String)
    where T: std::string::ToString {
    let mut param = Element::new("param");
    param.attributes.insert("value".to_string(), value.to_string());
    param.attributes.insert("name".to_string(), name);
    el.children.push(param)
}

pub fn mark_add<T>(el: &mut Element, value: T, name: String)
    where T: std::string::ToString {
    let mut mark = Element::new("mark");
    mark.attributes.insert("value".to_string(), value.to_string());
    mark.attributes.insert("name".to_string(), name);
    el.children.push(mark)
}
