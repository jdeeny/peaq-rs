#![feature(duration_float)]
use libsoxr;
use libsoxr::Soxr;

use failure::Error;
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use rodio;
use rodio::{Decoder, Source};

use peaq::Peaq;

#[derive(StructOpt, Debug)]
#[structopt(name = "wav_compare")]
struct Opt {
    /// Input audio file
    #[structopt(name = "REFERENCE", parse(from_os_str))]
    file_ref: PathBuf,

    /// Output file
    #[structopt(name = "TESTFILE", parse(from_os_str))]
    file_test: PathBuf,
}

fn make_resampler(in_rate: f64, out_rate: f64, channels: u32) -> Result<Soxr, Error> {
    use libsoxr::spec::QualityRecipe;
    use libsoxr::spec::QualityFlags;
    use libsoxr::datatype::Datatype;

    let io_spec = libsoxr::IOSpec::new(Datatype::Float64I, Datatype::Float64I);
    let quality_spec = libsoxr::QualitySpec::new(&QualityRecipe::VeryHigh, QualityFlags::HI_PREC_CLOCK);

    let x =
    Soxr::create(in_rate, out_rate, channels, Some(io_spec), Some(quality_spec), None);
    println!("{:?}", &x);
    Ok(x.unwrap())
}


fn make_source(path: &PathBuf) -> Result<Decoder<BufReader<File>>, Error> {
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file))?;
    Ok(source)
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let ref_decoder = make_source(&opt.file_ref)?;
    let test_decoder = make_source(&opt.file_ref)?;

    let ref_rate = ref_decoder.sample_rate() as f64;
    let ref_channels = ref_decoder.channels() as u32;
    let ref_duration = ref_decoder.total_duration().expect("Cannot determine duration of reference source.").as_float_secs();

    let test_rate = test_decoder.sample_rate() as f64;
    let test_channels = test_decoder.channels() as u32;
    let test_duration = test_decoder.total_duration().expect("Cannot determine duration of test source.").as_float_secs();

    println!("{} {} {}     {} {} {}", ref_rate, ref_channels, ref_duration, test_rate, test_channels, test_duration);
    let ref_soxr = make_resampler(ref_rate, peaq::SAMPLE_RATE, ref_channels)?;
    let test_soxr = make_resampler(test_rate, peaq::SAMPLE_RATE, test_channels)?;

    let peaq = Peaq::new(u32::max(ref_channels, test_channels));

    let ref_vec: Vec<f64> = ref_decoder.convert_samples().map(|s: f32| {s as f64}).collect();
    let test_vec: Vec<f64> = test_decoder.convert_samples().map(|s: f32| {s as f64}).collect();

    let result = peaq.compare(ref_channels, &ref_vec, test_channels, &test_vec);

    println!("Result: {:?}", result);
    Ok(())
}
