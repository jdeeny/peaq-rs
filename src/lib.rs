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


#[derive(Default)]
pub struct AudioSample {
}

#[derive(Default)]
pub struct PeaqScore {
    pub di: f64,
    pub odg: f64,
}

#[derive(Default)]
pub struct Peaq {

}

impl Peaq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compare(ref_in: &AudioSample, test_in: &AudioSample) -> Result<PeaqScore, Error> {
        Ok(PeaqScore::default())
    }
}
