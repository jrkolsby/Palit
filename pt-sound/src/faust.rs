use std::ffi::{OsStr};
use libcommon::{Action, dsp, UI};
use libloading::{Library, Symbol};
use libloading::os::unix::Symbol as RawSymbol;
use crate::core::{Output, CHANNELS, FRAMES};

// So we are going to build a struct which implements the UI trait
// ... and load an object from our compiled library which will
// ... implement dsp. We can store a these functions in our store
// ... which will act as a virtual table

// This opaque structure fills in for the mydsp class
#[repr(C)] pub struct Plugin { _private: [u8; 0] }
type New = extern "C" fn() -> *mut Plugin;
type Init = extern "C" fn(*mut Plugin, i32) -> ();
type Compute = unsafe extern "C" fn(*mut Plugin, i32, *const *const Output, *mut *mut Output) -> ();
type BuildUI = extern "C" fn(*mut Plugin, *mut PluginUI) -> ();

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
    /* DO WE NEED THSE FUNCTIONS?
    instanceInit: Symbol<extern fn(&Plugin, i32) -> ()>,
    getSampleRate: Symbol<extern fn(&Plugin) -> i32>;
    getNumInputs: Symbol<extern fn(&Plugin) -> i32>,
    getNumOutputs: Symbol<extern fn(&Plugin) -> i32>,
    getInputRate: Symbol<extern fn(&Plugin, i32) -> i32>,
    getOutputRate: Symbol<extern fn(&Plugin, i32) -> i32>,
    instanceResetUserInterface: Symbol<extern fn(&Plugin) -> ()>,
    instanceClear: Symbol<extern fn(&Plugin) -> ()>,
    instanceConstants: Symbol<extern fn(&Plugin, i32) -> ()>,
    */
}

impl PluginVTable {
    fn new(lib: &Library) -> Self {
        unsafe {
            let new: Symbol<New> = lib.get(b"newmydsp").unwrap();
            let init: Symbol<Init> = lib.get(b"initmydsp").unwrap();
            let compute: Symbol<Compute> = lib.get(b"computemydsp").unwrap();
            let buildUserInterface: Symbol<BuildUI> = lib.get(b"buildUserInterfacemydsp").unwrap();
            PluginVTable {
                new: new.into_raw(),
                init: init.into_raw(),
                compute: compute.into_raw(),
                buildUserInterface: buildUserInterface.into_raw(),
            }
        }
    }
}

impl dsp<Output> for Store {
    fn init(&mut self, samplingFreq: i32) {
        unsafe {
            (self.vtable.init)(self.object, samplingFreq)
        }
    }

    fn compute(&mut self, count: i32, inputs: &[&[Output]], outputs: &mut[&mut[Output]]) {
        unsafe {
            (self.vtable.compute)(self.object, count, inputs.as_ptr() as *const *const Output, outputs.as_mut_ptr() as *mut *mut Output)
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
    object: *mut Plugin,
}

pub fn init(lib_src: String) -> Store {
    let lib = Library::new(OsStr::new(&lib_src)).unwrap();
    let vtable = PluginVTable::new(&lib);
    Store {
        ui: PluginUI::new(),
        object: (vtable.new)(),
        vtable: vtable,
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
