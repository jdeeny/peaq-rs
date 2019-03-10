use failure::Error;
use structopt::StructOpt;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use rodio;
use rodio::decoder::Decoder;

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

    let mut ref_decoder = make_source(&opt.file_ref);
    let mut test_decoder = make_source(&opt.file_ref);

    let mut peaq = Peaq::new();

//    println!("{:?}\n\n{:?}", ref_spec, test_spec);

    //let result = peaq.compare(ref_reader.samples(), test_reader.samples());

    //println!("Result: {:?}", result);
    Ok(())
}
