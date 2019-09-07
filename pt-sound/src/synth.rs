use sample::{signal, Signal, Sample};

use wavefile::{WaveFile, WaveFileIterator};

use crate::core::{SF, SigGen};

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

pub struct Synth {
    pub sigs: Vec<Option<Sig>>,
    pub sample_rate: signal::Rate,
    pub stored_sample: Option<SF>,
    pub bar_values: [f64; 9],
}

impl Synth {
    pub fn add_note(&mut self, note: u8, vol: f64) {
        let hz = 440. * 2_f64.powf((note as f64 - 69.)/12.);

        for (baridx, barfreq) in BAR_FREQS.iter().enumerate() {
            let idx = self.sigs.iter().position(|s| s.is_none());
            let idx = if let Some(idx) = idx { idx } else {
                println!("Voice overflow!"); return;
            };
            let hz = self.sample_rate.const_hz(hz * 8. / barfreq);
            let s = Sig { sig: hz.sine(), note, targetvol: vol, curvol: 0., baridx };
            self.sigs[idx] = Some(s);
        }
    }
    pub fn remove_note(&mut self, note: u8) {
        for i in self.sigs.iter_mut() {
            if let &mut Some(ref mut i) = i {
                if i.note == note { i.targetvol = 0. }
            }
        }
    }
    pub fn cc(&mut self, ctrl: u32, value: i32) {
        let idx = match ctrl {
            // Standard knobs on UMA25S, modify to your liking
            1 => 0,
            74 => 1,
            71 => 2,
            73 => 3,
            75 => 4,
            72 => 5,
            91 => 6,
            93 => 7,
            10 => 8,
            _ => return,
        };
        self.bar_values[idx] = f64::from(value) / 255.;
    }
}

impl Iterator for Synth { 
    type Item = SF;
    fn next(&mut self) -> Option<Self::Item> {

        // Mono -> Stereo
        if let Some(s) = self.stored_sample.take() { return Some(s) };
        
        let mut z = 0f64;
        for sig in &mut self.sigs { 
            let mut remove = false;
            if let &mut Some(ref mut i) = sig {
                let barvalue = self.bar_values[i.baridx];
                if barvalue > 0.0 {
                    let s = i.sig.next();
                    z += s[0].mul_amp(i.curvol * barvalue);
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
        let z: Option<SF> = Some(SF::from_sample(z));
        self.stored_sample = z;
        z
    }
}

