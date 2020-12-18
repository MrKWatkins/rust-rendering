mod colour;
pub use self::colour::Colour;

pub mod image;
pub use self::image::Image;

pub mod rgb;
pub use self::rgb::Rgb;

pub mod rgb_image;
pub use self::rgb_image::RgbImage;
use snafu::Snafu;

use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to save image to {:?}; {}.", path, reason))]
    CannotSaveImage { path: PathBuf, reason: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
