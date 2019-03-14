use failure::Error;

pub use crate::leveladapter::LevelAdapter;
pub use crate::modulation::ModulationProcessor;

pub const SAMPLE_RATE: f64 = 48_000.;
pub const BLOCK_SIZE: usize = 4 * 1024;

use itertools::Itertools;

mod basic;
mod advanced;

use basic::BasicPeaq;
use advanced::AdvancedPeaq;


#[derive(Default, Debug)]
pub struct PeaqScore {
    pub di: f64,
    pub odg: f64,
}


pub struct Peaq {
    basic: Option<BasicPeaq>,
    advanced: Option<AdvancedPeaq>,
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

#[derive(PartialEq)]
pub enum PeaqLevel {
    Basic,
    Advanced
}

impl Peaq {
    pub fn new(channels: u32, level: PeaqLevel) -> Self {

        let ref_buffer = [0.; BLOCK_SIZE];
        let test_buffer = [0.; BLOCK_SIZE];
        let frame_count = 0;
        let basic_model = if level == PeaqLevel::Basic { None } else { Some(BasicPeaq::new()) };
        let advanced_model = if level == PeaqLevel::Advanced { None } else { Some(AdvancedPeaq::new()) };
        Self { basic: basic_model, advanced: advanced_model }
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
        if let Some(basic) = &self.basic {
            basic.process_frame(chunk);
        } else if let Some(advanced) = &self.advanced {
            advanced.process_frame(chunk);
        }
    }

}
