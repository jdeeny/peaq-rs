use crate::earmodel::*;

#[derive(Default)]
pub struct FFTEarModel {
    pub model_data: EarModelData,
}


static FRAMESIZE: u32 = 2048;

impl EarModel for FFTEarModel {
    fn new() -> Self {
        Self::default()

    }
    fn get_tau(&self) -> (f64, f64) {
        let tau_min = 0.008;
        let tau_100 = 0.030;
        (tau_min, tau_100)
    }
    fn get_loudness_scale(&self) -> f64 { 1.07664 }
    fn get_stepsize(&self) -> u32 { FRAMESIZE }
}

impl FFTEarModel {
    pub fn get_band_count(&self) -> u32 { 0 }
}
