// Attempt to clone the heart of gstpeaq in rust - https://github.com/HSU-ANT/gstpeaq

#[macro_use]
extern crate lazy_static;

mod test;
pub mod earmodel;
pub mod fbearmodel;
pub mod fftearmodel;
pub mod nn;

#[derive(Default)]
pub struct Peaq {

}

impl Peaq {
    pub fn new() -> Self {
        Self::default()
    }
}
