use failure::Error;
use structopt::StructOpt;
use std::path::PathBuf;

use hound;


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
    file_in: PathBuf,

    /// Output file
    #[structopt(name = "OUTPUT FILE", parse(from_os_str))]
    file_out: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut ref_reader = hound::WavReader::open(opt.file_in)?;
    let mut test_reader = hound::WavReader::open(opt.file_out)?;

    Ok(())
}
