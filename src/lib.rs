// Attempt to clone the heart of gstpeaq in rust - https://github.com/HSU-ANT/gstpeaq

#[macro_use]
extern crate lazy_static;


#[macro_use]
extern crate failure;
use failure::Error;

mod test;
pub mod peaq;
pub mod earmodel;
pub mod fbearmodel;
pub mod fftearmodel;
pub mod nn;
pub mod config;
pub mod leveladapter;
pub mod modulation;
pub mod measures;

pub use peaq::{Peaq, PeaqScore};
