use crate::image::{Error, Image, Rgb};
use image::codecs::png::PngEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;

pub struct RgbImage {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Rgb>,
}

impl RgbImage {
    pub fn new(width: u32, height: u32) -> RgbImage {
        return RgbImage {
            width,
            height,
            pixels: vec![Rgb::new(0, 0, 0); (width * height) as usize],
        };
    }

    pub fn from_rgb_pixels(width: u32, height: u32, pixels: &mut dyn Iterator<Item = Rgb>) -> RgbImage {
        return RgbImage {
            width,
            height,
            pixels: pixels.collect(),
        };
    }

    pub fn to_image(&self) -> Image {
        return Image::from_colour_pixels(self.width, self.height, &mut self.pixels.iter().map(|rgb| rgb.to_colour()));
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut buffer: Vec<u8> = Vec::with_capacity((self.width * self.height * 3) as usize);
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let color = &self.pixels[x * y];
                buffer.push(color.r);
                buffer.push(color.g);
                buffer.push(color.b);
            }
        }

        let file;
        match File::create(path) {
            Ok(f) => file = f,
            Err(e) => {
                return Err(Error::CannotSaveImage {
                    path: path.to_path_buf(),
                    reason: e.to_string(),
                })
            }
        }

        let encoder = PngEncoder::new(file);

        return match encoder.encode(&buffer, self.width, self.height, ColorType::Rgb8) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::CannotSaveImage {
                path: path.to_path_buf(),
                reason: e.to_string(),
            }),
        };
    }
}
