#![crate_type = "cdylib"]

use std::ffi::{OsStr, CStr};
use libc::{c_int, c_char, c_float};
use libcommon::{Action};
use libloading::{Library, Symbol};
use libloading::os::unix::Symbol as RawSymbol;
use crate::core::{Output, CHANNELS, FRAMES};

// So we are going to build a struct which implements the UI trait
// ... and load an object from our compiled library which will
// ... implement dsp. We can store a these functions in our store
// ... which will act as a virtual table. 
// A single instance of this struct will be able to process one set
// ... of inputs and outputs and so it will be known as a Voice

type DspNew = extern "C" fn() -> *mut Voice;
type DspInit = extern "C" fn(*mut Voice, c_int) -> ();
type DspCompute = unsafe extern "C" fn(*mut Voice, c_int, *const *const Output, *mut *mut Output) -> ();
type DspBuildUI = extern "C" fn(*mut Voice, *mut UIGlue) -> ();
type DspNumIO = extern "C" fn(*mut Voice) -> c_int;

type UIOpenBox = extern "C" fn(*mut PluginUI, *const c_char);
type UICloseBox = extern "C" fn(*mut PluginUI);
type UIAddButton = extern "C" fn (*mut PluginUI, *const c_char, *mut c_float);
type UIAddSlider = extern "C" fn (*mut PluginUI, 
    *const c_char,    // label
    *mut c_float,     // zone (mutable value)
    *const c_float,   // init
    *const c_float,   // min
    *const c_float,   // max
    *const c_float);  // step
type UIAddBargraph = extern "C" fn (*mut PluginUI, 
    *const c_char, 
    *mut c_float, 
    *const c_float,   // min
    *const c_float);  // max
type UIAddSoundFile = extern "C" fn (*mut PluginUI, *const c_char, *const c_char, *const *const SoundFile);
type UIDeclare = extern "C" fn (*mut PluginUI, *mut c_float, *const c_char, *const c_char);

extern "C" fn openTabBox(ui: *mut PluginUI, label: *const c_char) { 
    eprintln!("openHorizontalBox {:?}", label);
}
extern "C" fn openHorizontalBox(ui: *mut PluginUI, label: *const c_char) { 
    eprintln!("openHorizontalBox {:?}", label);
}
extern "C" fn openVerticalBox(ui: *mut PluginUI, label: *const c_char) { 
    eprintln!("openVerticalBox {:?}", label);
}
extern "C" fn closeBox(ui: *mut PluginUI) { 
    eprintln!("closeBox");
}
extern "C" fn addButton(ui: *mut PluginUI, label: *const c_char, param: *mut c_float) { 
    eprintln!("addButton {:?}", label);
}
extern "C" fn addCheckButton(ui: *mut PluginUI, label: *const c_char, param: *mut c_float) { 
    eprintln!("addCheckButton {:?}", label);
}
extern "C" fn addVerticalSlider(ui: *mut PluginUI, 
        label: *const c_char,
        param: *mut c_float,
        init: *const c_float,
        min: *const c_float,
        max: *const c_float,
        step: *const c_float) { 
    eprintln!("addVerticalSlider {:?}", label);
}
extern "C" fn addHorizontalSlider(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        init: *const c_float,
        min: *const c_float,
        max: *const c_float,
        step: *const c_float) { 
    eprintln!("addHorizontalSlider {:?}", label);
}
extern "C" fn addNumEntry(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        init: *const c_float,
        min: *const c_float,
        max: *const c_float,
        step: *const c_float) { 
    eprintln!("addNumEntry {:?}", label);
}
extern "C" fn addHorizontalBargraph(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        min: *const c_float,
        max: *const c_float) { 
    eprintln!("addHorizontalBargraph {:?}", label);
}
extern "C" fn addVerticalBargraph(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        min: *const c_float,
        max: *const c_float) {
    eprintln!("addVerticalBargraph {:?}", label);
}
extern "C" fn addSoundfile(ui: *mut PluginUI,
        foo: *const c_char, 
        bar: *const c_char,
        sf: *const *const SoundFile) { 
    eprintln!("addSoundFile");
}
extern "C" fn declare(ui: *mut PluginUI,
        param: *mut c_float,
        key: *const c_char,
        val: *const c_char) { 
    eprintln!("declare {:?} {:?}", key, val);
}

// These structs contain no data and stand in for void*
#[repr(C)] pub struct Voice { _private: [u8; 0] }
#[repr(C)] pub struct SoundFile { _private: [u8; 0] }
#[repr(C)] pub struct PluginUI { 
    name: String,
    _private: [u8; 0] 
}

// Plugin will access properties and functions from this struct
#[repr(C)] 
pub struct UIGlue { 
    uiInterface: *mut PluginUI,
    openTabBox: UIOpenBox,
    openHorizontalBox: UIOpenBox,
    openVerticalBox: UIOpenBox,
    closeBox: UICloseBox,
    addButton: UIAddButton,
    addCheckButton: UIAddButton,
    addVerticalSlider: UIAddSlider,
    addHorizontalSlider: UIAddSlider,
    addNumEntry: UIAddSlider,
    addHorizontalBargraph: UIAddBargraph,
    addVerticalBargraph: UIAddBargraph,
    addSoundfile: UIAddSoundFile,
    declare: UIDeclare,
}

impl PluginUI {
    fn new(name: String) -> Self {
        PluginUI { name, _private: [] }
    }
}

struct PluginVTable {
    new: RawSymbol<DspNew>,
    init: RawSymbol<DspInit>,
    compute: RawSymbol<DspCompute>,
    getNumInputs: RawSymbol<DspNumIO>,
    getNumOutputs: RawSymbol<DspNumIO>,
    buildUserInterface: RawSymbol<DspBuildUI>,
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
            let new: Symbol<DspNew> = lib.get(b"newmydsp").unwrap();
            let init: Symbol<DspInit> = lib.get(b"initmydsp").unwrap();
            let compute: Symbol<DspCompute> = lib.get(b"computemydsp").unwrap();
            let getNumInputs: Symbol<DspNumIO> = lib.get(b"getNumInputsmydsp").unwrap();
            let getNumOutputs: Symbol<DspNumIO> = lib.get(b"getNumOutputsmydsp").unwrap();
            let buildUserInterface: Symbol<DspBuildUI> = lib.get(b"buildUserInterfacemydsp").unwrap();
            PluginVTable {
                new: new.into_raw(),
                init: init.into_raw(),
                compute: compute.into_raw(),
                getNumInputs: getNumInputs.into_raw(),
                getNumOutputs: getNumOutputs.into_raw(),
                buildUserInterface: buildUserInterface.into_raw(),
            }
        }
    }
}

pub struct Store {
    ui: Box<PluginUI>,
    uiGlue: Box<UIGlue>,
    vtable: PluginVTable,
    voices: Vec<*mut Voice>,
    outputs: Vec<Vec<Output>>,
}

pub fn init(lib_src: String) -> Store {
    let lib = Library::new(OsStr::new(&lib_src)).unwrap();
    let vtable = PluginVTable::new(&lib);
    let mut ui = Box::new(PluginUI::new("Test".to_string()));
    let mut uiGlue = Box::new(UIGlue { 
        uiInterface: &mut *ui,
        openTabBox,
        openHorizontalBox,
        openVerticalBox,
        closeBox,
        addButton,
        addCheckButton,
        addVerticalSlider,
        addHorizontalSlider,
        addNumEntry,
        addHorizontalBargraph,
        addVerticalBargraph,
        addSoundfile,
        declare,
    });

    // Initialize voice
    let voice0 = (vtable.new)();
    (vtable.init)(voice0, 48000);
    eprintln!("did this work 1?");

    let num_inputs = (vtable.getNumInputs)(voice0) as usize;
    let num_outputs = (vtable.getNumOutputs)(voice0) as usize;
    eprintln!("{} INPUTS {} OUTPUTS", num_inputs, num_outputs);

    (vtable.buildUserInterface)(voice0, &mut *uiGlue);
    eprintln!("did this work 2?"); // NO!

    Store {
        ui,
        uiGlue,
        vtable,
        voices: vec![voice0],
        // Make sure we are not allocating this in tight loop (see below)
        outputs: vec![vec![0.0; FRAMES as usize]; num_outputs]
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
    let mut out_mut_ptr: Vec<*mut Output> = 
        store.outputs.iter_mut().map(|out| out.as_mut_ptr()
    ).collect();

    let out_const_ptr: Vec<*const Output> = store.outputs.iter().map(|out| 
        out.as_ptr()
    ).collect();

    // SEG FAULTING!
    unsafe {
        (store.vtable.compute)(
            store.voices[0],
            FRAMES as c_int, 
            out_const_ptr.as_ptr() as *const *const Output, 
            out_mut_ptr.as_mut_ptr() as *mut *mut Output,
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
