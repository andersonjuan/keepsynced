//use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
//use std::io;
//use std::path::Path;
//use std::ffi::OsStr;

mod configs;

/// target file to watch and type of operation to perform
#[derive(Debug, StructOpt)]
struct Cli {
    /// file to watch
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// upload or download flag
    #[structopt(short = "u", long = "upload")]
    upload: bool,
}

/// constants for saving metadata
const KEEPSYNCED_CONFIG_DIR: PathBuf = PathBuf::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let mut configs = configs::init(args.path, KEEPSYNCED_CONFIG_DIR);

    let metadata = fs::metadata(&args.path)?;
    let time = match metadata.modified() {
        Ok(time) => time,
        Err(error) => {
            return Err(error.into());
        }
    };

    configs.update(time);

    // TODO: complete the upload logic

    Ok(())
}
