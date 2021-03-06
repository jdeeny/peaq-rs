use failure::Error;
use std::default::Default;

#[derive(Default)]
pub struct AdvancedPeaq {
    config: AdvancedPeaqConfig,
    fft: (),

}


pub struct AdvancedPeaqConfig {
    framesize: u32,
    fs: f64,
}
impl Default for AdvancedPeaqConfig {
    fn default() -> Self {
        AdvancedPeaqConfig {
            framesize: 2048,
            fs: 48_000.
        }
    }
}

impl AdvancedPeaq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_frame<'a>(&self, chunk: impl Iterator<Item=(&'a f64,&'a f64)>) {

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
        //let stepsize = self.get_stepsize();
        // (21), (38), (41), and (56) in [BS1387], (32) in [Kabal03]
        let tau = tau_min + 100. / fc * (tau_100 - tau_min);
        // (24), (40), and (44) in [BS1387], (33) in [Kabal03]
        f64::exp(self.config.framesize as f64 / (-self.config.fs * tau))
    }

}
