use crate::earmodel::*;

#[derive(Default)]
pub struct FilterbankEarModel {
    pub model_data: EarModelData,
}

static FRAMESIZE: u32 = 192;

impl EarModel for FilterbankEarModel {
    fn new() -> Self {
        Self::default()
    }
    fn get_tau(&self) -> (f64, f64) {
        let tau_min = 0.004;
        let tau_100 = 0.020;
        (tau_min, tau_100)
    }
    fn get_loudness_scale(&self) -> f64 { 1.26539 }
    fn get_stepsize(&self) -> u32 { FRAMESIZE }
}
