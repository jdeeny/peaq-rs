use failure::Error;
use std::default::Default;


pub trait EarModel {
    fn new() -> Self;
    fn get_tau(&self) -> (f64, f64);
    fn get_loudness_scale(&self) -> f64;
    fn get_stepsize(&self) -> u32;
    fn process_block(&self) -> Result<(), Error> {
        Ok(())
    }
    fn make_bands(&self, band_fcs: &[f64]) -> Vec<Band> {
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
    }
    fn calc_time_constant(&self, fc: f64, tau_min: f64, tau_100: f64) -> f64 {
        let stepsize = self.get_stepsize();
        // (21), (38), (41), and (56) in [BS1387], (32) in [Kabal03]
        let tau = tau_min + 100. / fc * (tau_100 - tau_min);
        // (24), (40), and (44) in [BS1387], (33) in [Kabal03]
        f64::exp(stepsize as f64 / (-48000. * tau))
    }

}

#[derive(Default)]
pub struct Band {
    pub fc: f64,
    /// internal noise; (13) in [BS1387] (18) in [Kabal03]
    pub internal_noise: f64,

    pub time_constant: f64,
    /// excitation threshold; (60) in [BS1387], (70) in [Kabal03]
    pub excitation_threshold: f64,
    /// hreshold index; (61) in [BS1387], (69) in [Kabal03]
    pub threshold: f64,
    /// Loudness scaling factor; part of (58) in [BS1387], (69) in [Kabal03]
    pub loudness_factor: f64,
}

#[derive(Default)]
pub struct EarModelData {
    pub bands: Vec<Band>,

    frame_size: u32,
    step_size: u32,
    loudness_scale: f64,
    tau_min: f64,
    tau_100: f64,

}
