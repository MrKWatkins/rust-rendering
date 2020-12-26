use crate::image::{Colour, RgbImage};

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Colour>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        return Image {
            width,
            height,
            pixels: vec![Colour::new(0.0, 0.0, 0.0); (width * height) as usize],
        };
    }

    pub fn from_colour_pixels(width: u32, height: u32, pixels: &mut dyn Iterator<Item = Colour>) -> Image {
        return Image {
            width,
            height,
            pixels: pixels.collect(),
        };
    }

    pub fn to_rgb_image(&self) -> RgbImage {
        return RgbImage::from_rgb_pixels(self.width, self.height, &mut self.pixels.iter().map(|colour| colour.to_rgb()));
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        self.pixels[(x + y * self.width) as usize] = colour;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Colour {
        return self.pixels[(x + y * self.width) as usize];
    }
}
