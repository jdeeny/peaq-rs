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

        // We want to process the input in 2k blocks that overlap by 50%. The 'even' iterators are offset by 1kb
        // and are interleaved with the 'odd' iterators.
        let zeroes = [0.0f64; 1024];
        let padding = [0.0f64; 1024*2-1];

        let ref_even = zeroes.iter().chain(ref_in.iter().clone()).chain(padding.iter());
        let ref_odd = ref_in.iter().clone().chain(zeroes.iter()).chain(padding.iter());

        let test_even = zeroes.iter().chain(test_in.iter().clone()).chain(padding.iter());
        let test_odd = test_in.iter().clone().chain(zeroes.iter()).chain(padding.iter());

        let zipped_even = ref_even.zip(test_even).chunks(2048);
        let zipped_odd = ref_odd.zip(test_odd).chunks(2048);

        let chunked = zipped_even.into_iter().interleave(zipped_odd.into_iter());

        for chunk in chunked.into_iter() {
            self.process_frame(chunk);

        }

        Ok(PeaqScore::default())
    }

    fn process_frame<'a>(&self, chunk: impl Iterator<Item=(&'a f64,&'a f64)>)
    {
        for (r, t) in chunk.into_iter() {
            print!("{} {}  ", r, t);
        }
        println!("\n\n\n");
        // process earmodel
        // -> excitation patterns
        // time spreading
        // pattern adaptation
        // modulation pattern processing
        // loudness calculation
        // Calculate MOVs
        // Neural Network

    }

}
