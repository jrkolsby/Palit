use std::ffi::{OsStr};
use libc::c_int;
use libcommon::{Action, dsp, UI};
use libloading::{Library, Symbol};
use libloading::os::unix::Symbol as RawSymbol;
use crate::core::{Output, CHANNELS, FRAMES};

// So we are going to build a struct which implements the UI trait
// ... and load an object from our compiled library which will
// ... implement dsp. We can store a these functions in our store
// ... which will act as a virtual table. 
// A single instance of this struct will be able to process one set
// ... of inputs and outputs and so it will be known as a Voice

// This opaque structure fills in for the mydsp class
#[repr(C)] pub struct Voice { _private: [u8; 0] }
type New = extern "C" fn() -> *mut Voice;
type Init = extern "C" fn(*mut Voice, c_int) -> ();
type Compute = unsafe extern "C" fn(*mut Voice, c_int, *const *const Output, *mut *mut Output) -> ();
type BuildUI = extern "C" fn(*mut Voice, *mut PluginUI) -> ();
type NumIO = extern "C" fn(*mut Voice) -> c_int;

struct PluginUI {
    param_declarations: Vec<Action>,
}

impl PluginUI {
    fn new() -> Self {
        PluginUI {
            param_declarations: vec![]
        }
    }
}

impl UI<i32> for PluginUI {
    fn openTabBox(&mut self, label: &str) -> () {
        println!("openTabBox: {}", label);
    }
    fn openHorizontalBox(&mut self, label: &str) -> () {
        println!("openHorizontalBox: {}", label);
    }
    fn openVerticalBox(&mut self, label: &str) -> () {
        println!("openVerticalBox: {}", label);
    }
    fn closeBox(&mut self) -> () {
        println!("closeBox:");
    }

    // -- active widgets
    fn addButton(&mut self, label: &str, zone: &mut i32) -> () {
        println!("addButton: {}", label);
    }
    fn addCheckButton(&mut self, label: &str, zone: &mut i32) -> () {
        println!("addCheckButton: {}", label);
    }
    fn addVerticalSlider(&mut self, label: &str, zone: &mut i32, init: i32, min: i32, max: i32, step: i32) -> () {
        println!("addVerticalSlider: {}", label);
    }
    fn addHorizontalSlider(&mut self, label: &str, zone: &mut i32 , init: i32, min: i32, max: i32, step: i32) -> () {
        println!("addHorizontalSlider: {}", label);
    }
    fn addNumEntry(&mut self, label: &str, zone: &mut i32, init: i32, min: i32, max: i32, step: i32) -> () {
        println!("addNumEntry: {}", label);
    }

    // -- passive widgets
    fn addHorizontalBargraph(&mut self, label: &str, zone: &mut i32, min: i32, max: i32) -> () {
        println!("addHorizontalBargraph: {}", label);
    }
    fn addVerticalBargraph(&mut self, label: &str, zone: &mut i32, min: i32, max: i32) -> () {
        println!("addVerticalBargraph: {}", label);
    }

    // -- metadata declarations
    fn declare(&mut self, zone: &mut i32, key: &str, value: &str) -> () {
        println!("declare: {} {}", key, value);
    }
}

// * is a primitive pointer
struct PluginVTable {
    new: RawSymbol<New>,
    init: RawSymbol<Init>,
    compute: RawSymbol<Compute>,
    buildUserInterface: RawSymbol<BuildUI>,
    getNumInputs: RawSymbol<NumIO>,
    getNumOutputs: RawSymbol<NumIO>,
    /* DO WE NEED THSE FUNCTIONS?
    instanceInit: Symbol<extern fn(&Voice, i32) -> ()>,
    getSampleRate: Symbol<extern fn(&Voice) -> i32>;
    getInputRate: Symbol<extern fn(&Voice, i32) -> i32>,
    getOutputRate: Symbol<extern fn(&Voice, i32) -> i32>,
    instanceResetUserInterface: Symbol<extern fn(&Voice) -> ()>,
    instanceClear: Symbol<extern fn(&Voice) -> ()>,
    instanceConstants: Symbol<extern fn(&Voice, i32) -> ()>,
    */
}

impl PluginVTable {
    fn new(lib: &Library) -> Self {
        unsafe {
            let new: Symbol<New> = lib.get(b"newmydsp").unwrap();
            let init: Symbol<Init> = lib.get(b"initmydsp").unwrap();
            let compute: Symbol<Compute> = lib.get(b"computemydsp").unwrap();
            let buildUserInterface: Symbol<BuildUI> = lib.get(b"buildUserInterfacemydsp").unwrap();
            let getNumInputs: Symbol<NumIO> = lib.get(b"getNumInputsmydsp").unwrap();
            let getNumOutputs: Symbol<NumIO> = lib.get(b"getNumOutputsmydsp").unwrap();
            PluginVTable {
                new: new.into_raw(),
                init: init.into_raw(),
                compute: compute.into_raw(),
                buildUserInterface: buildUserInterface.into_raw(),
                getNumInputs: getNumInputs.into_raw(),
                getNumOutputs: getNumOutputs.into_raw(),
            }
        }
    }
}

impl dsp<Output> for Store {
    fn init(&mut self, samplingFreq: i32) {
        unsafe {
            (self.vtable.init)(self.voices[0], samplingFreq)
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Output]], outputs: &mut[&mut[Output]]) {
        unsafe {
            (self.vtable.compute)(self.voices[0], count, inputs.as_ptr() as *const *const Output, outputs.as_mut_ptr() as *mut *mut Output)
        }
    }

    fn buildUserInterface(&mut self, ui_interface: &UI<f32>) {}
    fn getSampleRate(&mut self) -> i32 { 0 }
    fn getNumInputs(&mut self) -> i32 { 1 }
    fn getNumOutputs(&mut self) -> i32 { 1 }
    fn getInputRate(&mut self, channel: i32) -> i32 { 0 }
    fn getOutputRate(&mut self, channel: i32) -> i32 { 0 }
    fn classInit(samplingFreq: i32) {}
    fn instanceResetUserInterface(&mut self) {}
    fn instanceClear(&mut self) {}
    fn instanceConstants(&mut self, samplingFreq: i32) {}
    fn instanceInit(&mut self, samplingFreq: i32) {}
}

pub struct Store {
    ui: PluginUI,
    vtable: PluginVTable,
    voices: Vec<*mut Voice>,
    outputs: Vec<Vec<Output>>,
}

pub fn init(lib_src: String) -> Store {
    let lib = Library::new(OsStr::new(&lib_src)).unwrap();
    let vtable = PluginVTable::new(&lib);
    let voice0 = (vtable.new)();
    let num_inputs = (vtable.getNumInputs)(voice0) as usize;
    let num_outputs = (vtable.getNumOutputs)(voice0);
    Store {
        ui: PluginUI::new(),
        voices: vec![voice0],
        vtable: vtable,
        // Make sure we are not allocating this in tight loop (see below)
        outputs: vec![vec![0.0; FRAMES as usize]; num_inputs]
    }
}

pub fn compute_buf(store: &mut Store, buffer: &&mut [[Output; CHANNELS]]) {
    // We need to prepare our channels as two seperate arrays
    // ... instead of using frames. This is just what faust wants *shrug*
    for (i, frame) in buffer.iter().enumerate() {
        for (j, sample) in frame.iter().enumerate() {
            if j < store.outputs.len() {
                store.outputs[j][i] = *sample;
            }
        }
    }
    let mut out_mut_ptr: Vec<*mut Output> = store.outputs.iter_mut().map(|out| out.as_mut_ptr()).collect();
    let out_const_ptr: Vec<*const Output> = store.outputs.iter().map(|out| out.as_ptr()).collect();

    unsafe {
        (store.vtable.compute)(
            store.voices[0], 
            FRAMES as i32, 
            out_const_ptr.as_ptr(), 
            out_mut_ptr.as_mut_ptr()
        );
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Output
        Option<Vec<Action>>, // Input
        Option<Vec<Action>>) { // Client 
    let mut client_actions: Vec<Action> = vec![];
    let mut output_actions: Vec<Action> = vec![];
    (None, None, None)
}
