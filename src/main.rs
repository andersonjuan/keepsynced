//use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let keep_path = init()?;
    let configs = configs::init(&args.path, &keep_path);

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

fn init() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // static config variables
    let home = "HOME";
    let file = "keepsynced.json"; // json for now...

    let home_path = match std::env::var_os(home) {
        Some(val) => val,
        None => {
            return Err("init: unable to find value for ENV []", home);
        }
    };

    let keep_path: PathBuf = [home_path, ".keepsynced"].iter().collect();

    if let Err(_) = fs::metadata(keep_path) {
        // create the dir
        keep_path = fs::create_dir(keep_path)?;
    }

    let mut config_path: PathBuf = keep_path.clone().push(file);

    if let Err(_) = fs::metadata(config_path) {
        config_path = fs::rename(config_path)?;
    }

    return Ok(config_path);
}
