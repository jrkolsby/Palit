use std::ffi::{OsStr, CStr};
use std::collections::HashMap;
use libc::{c_int, c_char, c_float};
use libcommon::{Action, note_to_hz};
use libloading::{Library, Symbol};
use libloading::os::unix::Symbol as RawSymbol;
use crate::core::{Output, CHANNELS, FRAMES};

// So we are going to build a struct which implements the UI trait
// ... and load an object from our compiled library which will
// ... implement dsp. We can store a these functions in our store
// ... which will act as a virtual table. 
// A single instance of this struct will be able to process one set
// ... of inputs and outputs and so it will be known as a Voice

const MAX_VOICES: usize = 12;

type DspNew = extern "C" fn() -> *mut Voice;
type DspInit = extern "C" fn(*mut Voice, c_int) -> ();
type DspCompute = unsafe extern "C" fn(*mut Voice, c_int, *const *const Output, *mut *mut Output) -> ();
type DspBuildUI = extern "C" fn(*mut Voice, *mut UIGlue) -> ();
type DspNumIO = extern "C" fn(*mut Voice) -> c_int;
type DspDelete = extern "C" fn(*mut Voice) -> ();

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

impl UIGlue {
    fn new(uiInterface: &mut PluginUI) -> Self {
        UIGlue { 
            uiInterface,
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
        }
    }
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
    delete: RawSymbol<DspDelete>,
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
            let delete: Symbol<DspDelete> = lib.get(b"deletemydsp").unwrap();
            PluginVTable {
                new: new.into_raw(),
                init: init.into_raw(),
                compute: compute.into_raw(),
                getNumInputs: getNumInputs.into_raw(),
                getNumOutputs: getNumOutputs.into_raw(),
                buildUserInterface: buildUserInterface.into_raw(),
                delete: delete.into_raw(),
            }
        }
    }
}

pub struct Store {
    lib: Library,
    vtable: PluginVTable,
    declarations: Vec<Action>,
    midi_enabled: bool,
    buffer: Vec<Vec<c_float>>,
    buffer_sum: Vec<Vec<c_float>>,
    voices: [Option<(*mut Voice, Box<PluginUI>, Box<UIGlue>)>; MAX_VOICES],
    next_voice: usize,
}

pub fn init(lib_src: String) -> Store {
    let lib = match Library::new(OsStr::new(&lib_src)) {
        Ok(lib) => lib,
        Err(_) => panic!("No such plugin {}", lib_src)
    };

    let vtable = PluginVTable::new(&lib);
    let mut declarations: Vec<Action> = vec![];

    let mut ui = Box::new(PluginUI::new());
    let mut uiGlue = Box::new(UIGlue::new(&mut *ui));

    // Initialize temp voice
    let voice0 = (vtable.new)();
    (vtable.init)(voice0, 48000);

    // Add anchor declaration
    let num_inputs = (vtable.getNumInputs)(voice0) as usize;
    let num_outputs = (vtable.getNumOutputs)(voice0) as usize;
    declarations.push(Action::DeclareAnchors(num_inputs, num_outputs));

    // Add UI declarations and free voice from C land
    (vtable.buildUserInterface)(voice0, &mut *uiGlue);
    declarations.extend(ui.declarations);
    (vtable.delete)(voice0);

    // Find these params in the declarations
    let midi_enabled = (declarations.iter().find(|&a| match a { 
        Action::DeclareParam(key,_,_,_,_) if key == "gate" => true, _ => false }).is_some() &&
        declarations.iter().find(|&a| match a {
        Action::DeclareParam(key,_,_,_,_) if key == "gain" => true, _ => false }).is_some() &&
        declarations.iter().find(|&a| match a {
        Action::DeclareParam(key,_,_,_,_) if key == "freq" => true, _ => false }).is_some());

    let store = Store {
        lib,
        vtable,
        voices: Default::default(),
        declarations,
        midi_enabled,
        // Make sure we are not allocating this in tight loop (see below)
        buffer: vec![vec![0.0; FRAMES as usize]; num_outputs],
        buffer_sum: vec![vec![0.0; FRAMES as usize]; num_outputs],
        next_voice: 0,
    };
    store
}

pub fn compute_buf(store: &mut Store, buffer: &mut [[Output; CHANNELS]]) {
    // We need to prepare our channels as two seperate arrays
    // ... instead of using frames. This is just what faust wants *shrug*
    for (i, frame) in buffer.iter().enumerate() {
        for (j, sample) in frame.iter().enumerate() {
            if j < store.buffer.len() {
                if store.midi_enabled {
                    // Midi enabled plugins don't take audio input
                    store.buffer_sum[j][i] = 0.0 as c_float;
                } else {
                    // Otherwise map into input buffer (will be overwritten)
                    store.buffer[j][i] = *sample as c_float;
                }
            }
        }
    }

    // Buffer will be passed to C for writing
    let mut output_ptrs: Vec<*mut Output> =
        store.buffer.iter_mut().map(|out| out.as_mut_ptr()
    ).collect();

    let input_ptrs: Vec<*const Output> = 
        store.buffer.iter().map(|out| out.as_ptr()
    ).collect();

    for voice in store.voices.iter() {

        if let Some((voice,_,_)) = voice {
            unsafe {
                (store.vtable.compute)(
                    *voice,
                    FRAMES as c_int, 
                    input_ptrs.as_ptr() as *const *const Output,
                    output_ptrs.as_mut_ptr() as *mut *mut Output,
                );
            }

            // Add voices together
            for i in 0..CHANNELS {
                for j in 0..FRAMES as usize {
                    store.buffer_sum[i][j] += store.buffer[i][j];
                }
            }
        }
    }

    // ... and then we have to copy the sum to the output frames *sigh*
    for i in 0..FRAMES as usize {
        for j in 0..CHANNELS {
            buffer[i][j] = store.buffer_sum[j][i] as f32;
        }
    }
}

pub fn dispatch(store: &mut Store, a: Action) {
    match a {
        Action::NoteOn(key, vel) => {
            if store.midi_enabled {
                if store.voices[store.next_voice].is_none() {
                    let new_voice = (store.vtable.new)();
                    (store.vtable.init)(new_voice, 48000);
                    let mut new_ui = Box::new(PluginUI::new());
                    let mut new_ui_glue = Box::new(UIGlue::new(&mut *new_ui));
                    (store.vtable.buildUserInterface)(new_voice, &mut *new_ui_glue);
                    unsafe { 
                        **new_ui.params.get_mut("freq").unwrap() = note_to_hz(key); 
                        **new_ui.params.get_mut("gain").unwrap() = vel as f32;
                        **new_ui.params.get_mut("gate").unwrap() = 1.0;
                    }
                    store.voices[store.next_voice] = Some((new_voice, new_ui, new_ui_glue));
                    // Find the next voice after us which either is_some() and gate = 0 
                    // ... or is_none()
                    'search: for (i, voice) in store.voices.iter().enumerate() {
                        if let Some((_, ui, _)) = voice {
                            unsafe {
                                if **ui.params.get("gate").unwrap() == 0.0 {
                                    store.next_voice = i;
                                    break 'search;
                                }
                            }
                        } else {
                            store.next_voice = i;
                            break 'search;
                        }
                    }
                } else {
                    if let Some(Some(ref mut voice)) = store.voices.get_mut(store.next_voice) {
                        unsafe { 
                            **voice.1.params.get_mut("freq").unwrap() = note_to_hz(key); 
                            **voice.1.params.get_mut("gain").unwrap() = vel as f32;
                            **voice.1.params.get_mut("gate").unwrap() = 1.0;
                        }
                    }
                };
            }
        },
        Action::NoteOff(key) => {
            if store.midi_enabled {
                'search: for (i, voice) in store.voices.iter_mut().enumerate() {
                    if let Some((_, ui, _)) = voice {
                        unsafe {
                            if **ui.params.get("freq").unwrap() == note_to_hz(key) {
                               **ui.params.get("gate").unwrap() = 0.0;
                                store.next_voice = i;
                            }
                        }
                    } else {
                        break 'search;
                    }
                }
            }
        },
        Action::SetParam(key, val) => {
            for voice in store.voices.iter_mut() {
                if let Some((_, ui, _)) = voice {
                    if let Some(param) = ui.params.get_mut(&key) {
                        unsafe { **param = val; }
                    }
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
    // This should only dispatch immediately after init
    let carry = if store.declarations.len() > 0 {
        Some(store.declarations.clone())
    } else {
        None
    };
    store.declarations.clear();
    (None, None, carry)
}