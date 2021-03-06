use failure::Error;
use std::default::Default;

use num_complex::Complex;

use std::sync::Arc;
use std::f64::consts::PI;

use rustfft::{FFTplanner, FFT};

pub struct BasicPeaq {
    config: BasicPeaqConfig,
    ref_fft: Arc<FFT<f64>>,
    test_fft: Arc<FFT<f64>>,
}


pub struct BasicPeaqConfig {
    framesize: usize,
    fs: f64,
    loudness_db: f64,
}

impl Default for BasicPeaqConfig {
    fn default() -> Self {
        BasicPeaqConfig {
            framesize: 2048,
            fs: 48_000.,
            loudness_db: 92.,
        }
    }
}

impl BasicPeaq {
    pub fn new() -> Self {
        let mut planner = FFTplanner::new(false);
        let config = BasicPeaqConfig::default();
        let ref_fft = planner.plan_fft(config.framesize / 2 - 1);
        let test_fft = planner.plan_fft(config.framesize / 2 - 1);

        Self {
            config: BasicPeaqConfig::default(),
            ref_fft: ref_fft,
            test_fft: test_fft,
        }
    }

    fn window(&self, n: f64) -> f64 {
        0.5 * (1. - f64::cos( (2. * PI * n) / (self.config.framesize as f64 -1.) ))
    }
    pub fn process_frame<'a>(&self, chunk: impl Iterator<Item=(&'a f64,&'a f64)>) {
        let mut inp: [Complex<f64>;2048] = [Complex::new(0., 0.);2048];
        let mut out: [Complex<f64>;2048] = [Complex::new(0., 0.);2048];
        self.ref_fft.process(&mut inp, &mut out);
        self.test_fft.process(&mut inp, &mut out);
    }

    // -> excitation patterns
    // time spreading
    // pattern adaptation
    // modulation pattern processing
    // loudness calculation
    // Calculate MOVs
    // Neural Network


    fn get_tau(&self) -> (f64, f64) {
        let tau_min = 0.008;
        let tau_100 = 0.030;
        (tau_min, tau_100)
    }
    fn get_loudness_scale(&self) -> f64 { 1.07664 }
    pub fn get_band_count(&self) -> u32 { 0 }
    /*fn make_bands(&self, band_fcs: &[f64]) -> Vec<Band> {
        let band_vec: Vec<Band> = band_fcs.iter().map(|fc| {
            let fc = *fc;
            let internal_noise = f64::powf(10., 0.4 * 0.364 * f64::powf(fc / 1000., -0.8));
            let excitation_threshold = f64::powf(10., 0.364 * f64::powf(fc / 1000., -0.8));
            let threshold = f64::powf(10., 0.1 * (-2. - 2.05 * f64::atan(fc / 4000.) -
                      0.75 * f64::atan (fc / 1600. * fc / 1600.)));
            let loudness_factor = self.get_loudness_scale() * f64::powf(excitation_threshold / (10_000. * threshold), 0.23);
            let (tau_min, tau_100) = self.get_tau();
            let time_constant = self.calc_time_constant(fc, tau_min, tau_100);
            Band { fc, internal_noise, excitation_threshold, threshold, loudness_factor, time_constant }
        }).collect();
        band_vec
    }*/
    fn calc_time_constant(&self, fc: f64, tau_min: f64, tau_100: f64) -> f64 {
        // (21), (38), (41), and (56) in [BS1387], (32) in [Kabal03]
        let tau = tau_min + 100. / fc * (tau_100 - tau_min);
        // (24), (40), and (44) in [BS1387], (33) in [Kabal03]
        f64::exp(self.config.framesize as f64 / (-self.config.fs * tau))
    }

}
