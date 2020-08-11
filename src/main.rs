use structopt::StructOpt;

/// target file to watch and type of operation to perform
#[derive(Debug, StructOpt)]
struct Cli {
    /// file to watch
    #[structopt(parse(from_os_str))]
    path: td::path::PathBuf,
    /// upload or download flag
    #[structopt(short = "u", long = "upload")]
    upload: Boolean,
}

/// constants for saving metadata
const KEEPSYNCED_CONFIG_DIR: &str = ""


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let mut configs = configs::init(&args.path);

    let metadata = fs::metadata(&args.path)?;
    let time = match metadata.modified() {
        Ok(time) => { time },
        Err(error) => { return Err(error.into()); }
    };

    configs.update(time)

    // TODO: complete the upload logic

    Ok(());
}
