#![feature(duration_float)]

use failure::Error;
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use rodio;
use rodio::{Decoder, Source};

use peaq::Peaq;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Input audio file
    #[structopt(name = "INPUT FILE", parse(from_os_str))]
    file_ref: PathBuf,

    /// Output file
    #[structopt(name = "OUTPUT FILE", parse(from_os_str))]
    file_test: PathBuf,
}


fn make_source(path: &PathBuf) -> Result<Decoder<BufReader<File>>, Error> {
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file))?;
    Ok(source)
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut ref_decoder = make_source(&opt.file_ref)?;
    let mut test_decoder = make_source(&opt.file_ref)?;

    let ref_rate = ref_decoder.sample_rate() as f64;
    let ref_channels = ref_decoder.channels() as u32;
    let ref_duration = ref_decoder.total_duration().expect("Cannot determine duration of reference source.").as_float_secs();

    let test_rate = test_decoder.sample_rate() as f64;
    let test_channels = test_decoder.channels() as u32;
    let test_duration = test_decoder.total_duration().expect("Cannot determine duration of test source.").as_float_secs();

    println!("{} {} {}     {} {} {}", ref_rate, ref_channels, ref_duration, test_rate, test_channels, test_duration);

    let mut peaq = Peaq::new();

    let ref_vec: Vec<f64> = ref_decoder.convert_samples().map(|s: f32| {s as f64}).collect();
    let test_vec: Vec<f64> = test_decoder.convert_samples().map(|s: f32| {s as f64}).collect();

    let result = peaq.compare(ref_rate, ref_channels, &ref_vec, test_rate, test_channels, &test_vec);

    //println!("Result: {:?}", result);
    Ok(())
}
