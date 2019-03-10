// Attempt to clone the heart of gstpeaq in rust - https://github.com/HSU-ANT/gstpeaq

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;
use failure::Error;

use libsoxr;
use libsoxr::Soxr;


mod test;
pub mod earmodel;
pub mod fbearmodel;
pub mod fftearmodel;
pub mod nn;
pub mod config;

const PEAQ_RATE: f64 = 48_000.;
const BLOCK_SIZE: usize = 4 * 1024;

#[derive(Default)]
pub struct PeaqScore {
    pub di: f64,
    pub odg: f64,
}

pub struct Peaq {
    ref_buffer: [f64; BLOCK_SIZE],
    test_buffer: [f64; BLOCK_SIZE],
}

impl Peaq {
    pub fn new() -> Self {
        let ref_buffer = [0.; BLOCK_SIZE];
        let test_buffer = [0.; BLOCK_SIZE];
        Self { ref_buffer, test_buffer }
    }

    pub fn compare(&self, ref_rate: f64, ref_ch: u32, ref_in: &[f64], test_rate: f64, test_ch: u32, test_in: &[f64]) -> Result<PeaqScore, Error> {
        let ref_soxr = Soxr::create(ref_rate, PEAQ_RATE, ref_ch, None, None, None)?;
        let test_soxr = Soxr::create(test_rate, PEAQ_RATE, test_ch, None, None, None)?;

        self.compare_native(ref_in, test_in)
    }

    // Compare native format (f64 @ 48khz) streams
    fn compare_native(&self, ref_in: &[f64], test_in: &[f64]) -> Result<PeaqScore, Error> {
        Ok(PeaqScore::default())
    }

    fn make_resampler(in_rate: f64, out_rate: f64, channels: u32) -> Result<Soxr, Error> {
        use libsoxr::spec::QualityRecipe;
        use libsoxr::spec::QualityFlags;
        use libsoxr::datatype::Datatype;

        let io_spec = libsoxr::IOSpec::new(Datatype::Float64I, Datatype::Float64I);
        let quality_spec = libsoxr::QualitySpec::new(&QualityRecipe::VeryHigh, QualityFlags::HI_PREC_CLOCK);

        Ok(Soxr::create(in_rate, out_rate, channels, Some(io_spec), Some(quality_spec), None).unwrap())

    }

    fn process_block(&self, ref_in: &[f64], test_in: &[f64]) {

    }

}
