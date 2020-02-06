use std::sync::{Arc, Mutex};
use std::path::Path;
use vst::host::{Host, PluginLoader, PluginInstance};
use vst::plugin::{Plugin};
use xmltree::Element;

use crate::core::{FRAMES, CHANNELS, Output};

pub struct VSTHost;

impl Host for VSTHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

pub struct Store {
    src: String,
    // PluginInstance implements Plugin
    plugin: PluginInstance,
    host: Arc<Mutex<VSTHost>>,
}

pub fn read(el: &mut Element) -> Option<Store> {
    let src = match el.attributes.get("src") {
        Some(val) => val.to_string(),
        None => return None
    };
    let store = init(src);
    Some(store)
}

/*
pub fn compute_buf(store: Store, buffer: [[Output; CHANNELS]; FRAMES]) {
    let mut host_buffer: HostBuffer<f32> = HostBuffer::new(2, 2);
    let inputs = vec![vec![0.0; 1000]; 2];
    let mut outputs = vec![vec![0.0; 1000]; 2];
    let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);
    dsp::slice::map_in_place(buffer, |a| {
        store.plugin.process()
    })

}
*/

pub fn init(src: String) -> Store {
    let host = Arc::new(Mutex::new(VSTHost));
    let path = Path::new(&src);

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut instance = loader.instance().unwrap();

    instance.init();

    Store { 
        src, 
        host,
        plugin: instance
    }
}