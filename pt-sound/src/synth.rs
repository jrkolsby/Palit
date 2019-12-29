use std::{iter};

use sample::{signal, Signal, Sample};
use wavefile::{WaveFile, WaveFileIterator};
use xmltree::Element;

use crate::document::{param_map};
use crate::core::{SF, SigGen, Output};
use crate::action::Action;

// Standard Hammond drawbar.
const BAR_FREQS: [f64; 9] = [16., 5.+1./3., 8., 4., 2.+2./3., 2., 1.+3./5., 1.+1./3., 1.];

#[derive(Clone)]
pub struct Sig {
    note: u8,
    sig: SigGen,
    targetvol: f64,
    curvol: f64,
    baridx: usize,
}

pub struct Store {
    pub queue: Vec<Action>,
    pub sigs: Vec<Option<Sig>>,
    pub sample_rate: signal::Rate,
    pub stored_sample: Option<Output>,
    pub bar_values: [f64; 9],
}

pub fn init() -> Store {
    Store {
        queue: vec![],
        sigs: iter::repeat(None).take(256).collect(),
        sample_rate: signal::rate(f64::from(44_100)),
        stored_sample: None,
        bar_values: [0.25, 0.25, 0.25, 0.75, 0.5, 0., 0., 0., 0.],
    }
}

pub fn dispatch(store: &mut Store, action: Action) {
    eprintln!("{:?}", action);
    match action {
        Action::NoteOnAt(_, note, vol) |
        Action::NoteOn(note, vol) => {
            let hz = 440. * 2_f64.powf((note as f64 - 69.)/12.);

            for (baridx, barfreq) in BAR_FREQS.iter().enumerate() {
                let idx = store.sigs.iter().position(|s| s.is_none());
                let idx = if let Some(idx) = idx { idx } else {
                    println!("Voice overflow!"); return;
                };
                let hz = store.sample_rate.const_hz(hz * 8. / barfreq);
                let s = Sig { sig: hz.sine(), note, targetvol: vol, curvol: 0., baridx };
                store.sigs[idx] = Some(s);
            }
            // Only carry NoteOn/Off actions, NOT NoteOnAt
            match action {
                Action::NoteOn(_, _) => store.queue.push(action),
                _ => {}
            };
        },
        Action::NoteOffAt(_, note) |
        Action::NoteOff(note) => {
            for i in store.sigs.iter_mut() {
                if let &mut Some(ref mut i) = i {
                    if i.note == note { i.targetvol = 0. }
                }
            }
            match action {
                Action::NoteOff(_) => store.queue.push(action),
                _ => {}
            };
        },
        Action::SetParam(_, ctrl, value) => {
            let idx = match ctrl.as_ref() {
                "16"    => 0,
                "5.3"   => 1,
                "8"     => 2,
                "4"     => 3,
                "2.6"   => 4,
                "2"     => 5,
                "1.6"   => 6,
                "1.3"   => 7,
                "1"     => 8,
                _ => return,
            };
            store.bar_values[idx] = f64::from(value) / 255.;
        }
        _ => {}
    }
}

pub fn read(doc: &mut Element) -> Option<Store> {
    let (_, params) = param_map(doc);
    let mut store = init();
    store.bar_values = [
        *params.get("16").unwrap() as f64 / 255.,
        *params.get("5.3").unwrap() as f64 / 255.,
        *params.get("8").unwrap() as f64 / 255.,
        *params.get("4").unwrap() as f64 / 255.,
        *params.get("2.6").unwrap() as f64 / 255.,
        *params.get("2").unwrap() as f64 / 255.,
        *params.get("1.6").unwrap() as f64 / 255.,
        *params.get("1.3").unwrap() as f64 / 255.,
        *params.get("1").unwrap() as f64 / 255.
    ];
    Some(store)
}

pub fn compute(store: &mut Store) -> Output {
    // Mono -> Stereo
    if let Some(s) = store.stored_sample.take() { return s };
    
    let mut z = 0f32;
    for sig in &mut store.sigs { 
        let mut remove = false;
        if let &mut Some(ref mut i) = sig {
            let barvalue = store.bar_values[i.baridx];
            if barvalue > 0.0 {
                let s = i.sig.next();
                z += s[0].mul_amp(i.curvol * barvalue) as f32;
            }

            // Quick and dirty volume envelope to avoid clicks. 
            if i.curvol != i.targetvol {
                if i.targetvol == 0. {
                    i.curvol -= 0.002;
                    if i.curvol <= 0. { remove = true; }
                } else {
                    i.curvol += 0.002;
                    if i.curvol >= i.targetvol { i.curvol = i.targetvol; }
                }
            }
        }
        if remove { *sig = None };
    }
    let z = z.min(0.999).max(-0.999);
    store.stored_sample = Some(z);
    z
}

pub fn dispatch_requested(store: &mut Store) -> (
        Option<Vec<Action>>, // Actions for outputs
        Option<Vec<Action>>, // Actions for inputs
        Option<Vec<Action>> // Actions for client
    ) {
        let carry = store.queue.clone();
        store.queue.clear();
        (None, None, Some(carry.clone()))
}