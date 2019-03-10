use failure::Error;
use std::default::Default;

pub trait EarModel {
    fn new() -> Self;
    fn process_block() -> Result<(), Error>;
}

#[derive(Default)]
pub struct EarModelData {
    pub band_count: u32,
    pub fc: f64,
    pub internal_noise: f64,
    pub ear_time_constants: f64,
    pub excitation_threshold: f64,
    pub threshold: f64,
    pub loudness_factor: f64,

    frame_size: u32,
    step_size: u32,
    loudness_scale: f64,
    tau_min: f64,
    tau_100: f64,

}

#[derive(Default)]
pub struct FFTEarModel {
    pub model_data: EarModelData,
}


#[derive(Default)]
pub struct FilterbankEarModel {
    pub model_data: EarModelData,
}


impl EarModel for FFTEarModel {
    fn new() -> Self {
        Self::default()
    }
    fn process_block() -> Result<(), Error> {
        Ok(())
    }
}

impl EarModel for FilterbankEarModel {
    fn new() -> Self {
        Self::default()
    }
    fn process_block() -> Result<(), Error> {
        Ok(())
    }
}
