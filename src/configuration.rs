use snafu::Snafu;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-rendering")]
pub struct Configuration {
    /// The output file.
    #[structopt(parse(from_os_str), short = "s", long = "scene")]
    pub scene: PathBuf,

    /// The output file.
    #[structopt(parse(from_os_str), short = "o", long = "output")]
    pub output: PathBuf,

    /// The width of the output image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    pub width: u32,

    /// The height of the output image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    pub height: u32,
}

pub fn from_command_line() -> Result<Configuration> {
    let configuration = Configuration::from_args();
    validate(&configuration)?;
    return Ok(configuration);
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The output file {:?} is invalid; {}.", output, reason))]
    OutputInvalid { output: PathBuf, reason: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn validate(configuration: &Configuration) -> Result<()> {
    if configuration.output.is_dir() {
        return Err(Error::OutputInvalid {
            output: configuration.output.clone(),
            reason: "path corresponds to a directory".to_string(),
        });
    }

    return Ok(());
}
