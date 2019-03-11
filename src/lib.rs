// Attempt to clone the heart of gstpeaq in rust - https://github.com/HSU-ANT/gstpeaq

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;
use failure::Error;

mod test;
pub mod earmodel;
pub mod fbearmodel;
pub mod fftearmodel;
pub mod nn;
pub mod config;
pub mod leveladapter;
pub mod modulation;
pub mod movs;

pub const SAMPLE_RATE: f64 = 48_000.;
pub const BLOCK_SIZE: usize = 4 * 1024;

pub use earmodel::EarModel;
pub use fftearmodel::FFTEarModel;
pub use fbearmodel::FilterbankEarModel;
pub use leveladapter::LevelAdapter;
pub use modulation::ModulationProcessor;

#[derive(Default, Debug)]
pub struct PeaqScore {
    pub di: f64,
    pub odg: f64,
}

pub struct Peaq {
    frame_count: u64,
    channels: u32,
    earmodel_fft: FFTEarModel,
    earmodel_fb: FilterbankEarModel,
    level_adapter: LevelAdapter,
}

impl Peaq {
    pub fn new(channels: u32) -> Self {
        let ref_buffer = [0.; BLOCK_SIZE];
        let test_buffer = [0.; BLOCK_SIZE];
        let frame_count = 0;
        let earmodel_fft = FFTEarModel::new();
        let earmodel_fb = FilterbankEarModel::new();
        let level_adapter = LevelAdapter::new(&earmodel_fft);
        Self { frame_count, channels, earmodel_fft, earmodel_fb, level_adapter }
    }

    pub fn compare(&self, ref_ch: u32, ref_in: &[f64], test_ch: u32, test_in: &[f64]) -> Result<PeaqScore, Error> {


        Ok(PeaqScore::default())
    }

    fn process_frame(&self, ref_in: &[f64], test_in: &[f64]) {

    }

}
