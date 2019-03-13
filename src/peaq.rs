use failure::Error;

pub use crate::earmodel::EarModel;
pub use crate::fftearmodel::FFTEarModel;
pub use crate::fbearmodel::FilterbankEarModel;
pub use crate::leveladapter::LevelAdapter;
pub use crate::modulation::ModulationProcessor;

pub const SAMPLE_RATE: f64 = 48_000.;
pub const BLOCK_SIZE: usize = 4 * 1024;

use itertools::Itertools;

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
        let zeroes = [0.0f64; 1024];
        let no_offset = ref_in.clone().iter().chain(zeroes.iter()).chunks(2048);
        let offset = zeroes.iter().chain(ref_in.clone()).chunks(2048);
        let interleaved = no_offset.into_iter().interleave(offset.into_iter());
        for x in interleaved.into_iter() {
            for y in x {
                println!("{:?}", y);
            }
        }
        //println!("{:?}", &windowed);

        Ok(PeaqScore::default())
    }

    /*fn process_frame(&self, ref_in: &[[f64; BLOCK_SIZE], test_in: &[f64]) {
        // process earmodel
        // -> excitation patterns
        // time spreading
        // pattern adaptation
        // modulation pattern processing
        // loudness calculation
        // Calculate MOVs
        // Neural Network

    }*/

}
