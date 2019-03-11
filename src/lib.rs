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

pub const SAMPLE_RATE: f64 = 48_000.;
pub const BLOCK_SIZE: usize = 4 * 1024;

#[derive(Default, Debug)]
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

    pub fn compare(&self, ref_ch: u32, ref_in: &[f64], test_ch: u32, test_in: &[f64]) -> Result<PeaqScore, Error> {

        Ok(PeaqScore::default())
    }

    fn process_block(&self, ref_in: &[f64], test_in: &[f64]) {

    }

}
