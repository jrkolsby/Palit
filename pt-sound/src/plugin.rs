use std::ffi::{OsStr, CStr};
use std::collections::HashMap;
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
    c_float,   // init
    c_float,   // min
    c_float,   // max
    c_float);  // step
type UIAddBargraph = extern "C" fn (*mut PluginUI, 
    *const c_char, 
    *mut c_float, 
    c_float,   // min
    c_float);  // max
type UIAddSoundFile = extern "C" fn (*mut PluginUI, *const c_char, *const c_char, *const *const SoundFile);
type UIDeclare = extern "C" fn (*mut PluginUI, *mut c_float, *const c_char, *const c_char);

extern "C" fn openTabBox(ui: *mut PluginUI, label: *const c_char) { 
    //eprintln!("openHorizontalBox {:?}", label);
}
extern "C" fn openHorizontalBox(ui: *mut PluginUI, label: *const c_char) { 
    //eprintln!("openHorizontalBox {:?}", label);
}
extern "C" fn openVerticalBox(ui: *mut PluginUI, label: *const c_char) { 
    //eprintln!("openVerticalBox {:?}", label);
}
extern "C" fn closeBox(ui: *mut PluginUI) { 
    //eprintln!("closeBox");
}
extern "C" fn addButton(ui: *mut PluginUI, label: *const c_char, param: *mut c_float) { 
    unsafe {
        let label_str = CStr::from_ptr(label).to_str().unwrap();
        let mut params = &mut (*ui).params;
        let mut declarations = &mut (*ui).declarations;
        params.insert(label_str.to_string(), param);
        declarations.push(Action::DeclareParam(
            label_str.to_string(),
            0.0,
            0.0,
            1.0,
            1.0,
        ));
    }
}
extern "C" fn addCheckButton(ui: *mut PluginUI, label: *const c_char, param: *mut c_float) { 
    //eprintln!("addCheckButton {:?}", label);
}
extern "C" fn addVerticalSlider(ui: *mut PluginUI, 
        label: *const c_char,
        param: *mut c_float,
        init: c_float,
        min: c_float,
        max: c_float,
        step: c_float) { 
    unsafe {
        let label_str = CStr::from_ptr(label).to_str().unwrap();
        let mut params = &mut (*ui).params;
        let mut declarations = &mut (*ui).declarations;
        params.insert(label_str.to_string(), param);
        declarations.push(Action::DeclareParam(
            label_str.to_string(),
            init as f32,
            min as f32,
            max as f32,
            step as f32,
        ));
    }
}
extern "C" fn addHorizontalSlider(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        init: c_float,
        min: c_float,
        max: c_float,
        step: c_float) { 
    unsafe {
        let label_str = CStr::from_ptr(label).to_str().unwrap();
        let mut params = &mut (*ui).params;
        let mut declarations = &mut (*ui).declarations;
        params.insert(label_str.to_string(), param);
        declarations.push(Action::DeclareParam(
            label_str.to_string(),
            init as f32,
            min as f32,
            max as f32,
            step as f32,
        ));
    }
}
extern "C" fn addNumEntry(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        init: c_float,
        min: c_float,
        max: c_float,
        step: c_float) { 
    //eprintln!("addNumEntry {:?}", label);
}
extern "C" fn addHorizontalBargraph(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        min: c_float,
        max: c_float) { 
    //eprintln!("addHorizontalBargraph {:?}", label);
}
extern "C" fn addVerticalBargraph(ui: *mut PluginUI,
        label: *const c_char,
        param: *mut c_float,
        min: c_float,
        max: c_float) {
    //eprintln!("addVerticalBargraph {:?}", label);
}
extern "C" fn addSoundfile(ui: *mut PluginUI,
        foo: *const c_char, 
        bar: *const c_char,
        sf: *const *const SoundFile) { 
    //eprintln!("addSoundFile");
}
extern "C" fn declare(ui: *mut PluginUI,
        param: *mut c_float,
        key: *const c_char,
        val: *const c_char) { 
    //eprintln!("declare {:?} {:?}", key, val);
}

// These structs contain no data and stand in for void*
#[repr(C)] pub struct Voice { _private: [u8; 0] }
#[repr(C)] pub struct SoundFile { _private: [u8; 0] }
#[repr(C)] pub struct PluginUI { 
    params: HashMap<String, *mut c_float>,
    declarations: Vec<Action>,
    _private: [u8; 0],
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
    fn new() -> Self {
        PluginUI { 
            params: HashMap::new(),
            declarations: vec![],
            _private: [] 
        }
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
    lib: Library,
    ui: Box<PluginUI>,
    uiGlue: Box<UIGlue>,
    vtable: PluginVTable,
    voices: Vec<*mut Voice>,
    outputs: Vec<Vec<c_float>>,
}

pub fn init(lib_src: String) -> Store {
    let lib = Library::new(OsStr::new(&lib_src)).unwrap();
    let vtable = PluginVTable::new(&lib);
    let mut ui = Box::new(PluginUI::new());
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

    let num_inputs = (vtable.getNumInputs)(voice0) as usize;
    let num_outputs = (vtable.getNumOutputs)(voice0) as usize;
    ui.declarations.push(Action::DeclareAnchors(num_inputs, num_outputs));

    (vtable.buildUserInterface)(voice0, &mut *uiGlue);

    Store {
        lib,
        ui,
        uiGlue,
        vtable,
        voices: vec![voice0],
        // Make sure we are not allocating this in tight loop (see below)
        outputs: vec![vec![0.0; FRAMES as usize]; num_outputs]
    }
}

// These values are really small, like 0.1
pub fn compute_buf(store: &mut Store, buffer: &mut [[Output; CHANNELS]]) {
    // We need to prepare our channels as two seperate arrays
    // ... instead of using frames. This is just what faust wants *shrug*
    for (i, frame) in buffer.iter().enumerate() {
        for (j, sample) in frame.iter().enumerate() {
            if j < store.outputs.len() {
                store.outputs[j][i] = *sample as c_float;
            }
        }
    }

    let mut out_mut_ptr: Vec<*mut Output> =
        store.outputs.iter_mut().map(|out| out.as_mut_ptr()
    ).collect();

    let out_const_ptr: Vec<*const Output> = store.outputs.iter().map(|out|
        out.as_ptr()
    ).collect();

    unsafe {
        (store.vtable.compute)(
            store.voices[0],
            FRAMES as c_int, 
            out_const_ptr.as_ptr() as *const *const Output,
            out_mut_ptr.as_mut_ptr() as *mut *mut Output,
        );
    }

    // ... and then we have to map back to frames *sigh*
    for i in 0..FRAMES as usize {
        for j in 0..CHANNELS {
            buffer[i][j] = store.outputs[j][i];
        }
    }
}

pub fn dispatch(store: &mut Store, a: Action) {
    match a {
        Action::SetParam(key, val) => {
            if let Some(param) = store.ui.params.get_mut(&key) {
                unsafe {
                    **param = val;
                }
            }
        },
        _ => {}
    }
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Output
        Option<Vec<Action>>, // Input
        Option<Vec<Action>>) { // Client 
    let decls = unsafe {
        &mut (*store.ui).declarations
    };
    let carry = if decls.len() > 0 {
        Some(decls.clone())
    } else {
        None
    };
    decls.clear();
    (None, None, carry)
}