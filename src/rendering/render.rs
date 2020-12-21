use crate::configuration::Configuration;
use crate::image::{Colour, Image};
use crate::maths::Coordinates;
use crate::rendering::algorithms::Algorithm;
use crate::rendering::{RasterSpace, SubPixelSampling};
use crate::scene::Scene;

pub fn render<TAlgorithm: Algorithm>(algorithm: &TAlgorithm, configuration: &Configuration, scene: &Scene, sampling: SubPixelSampling) -> Image {
    let mut image = Image::new(configuration.width, configuration.height);

    let raster_space = RasterSpace::new(configuration.width, configuration.height);

    let sampling_offsets = sampling.pixel_offsets();

    let mut samples: Vec<Colour> = sampling_offsets.iter().map(|_| Colour::black()).collect();

    for x in 0..configuration.width {
        for y in 0..configuration.height {
            for sample in 0..sampling_offsets.len() {
                let offset = &sampling_offsets[sample];

                let raster_coords = Coordinates::new(x as f32 + offset.x, y as f32 + offset.y);

                let screen_space_coords = raster_space.to_screen_space(raster_coords);

                samples[sample] = algorithm.render_point(scene, &screen_space_coords);
            }
            image.set_pixel(x, y, Colour::average(&samples));
        }
    }

    return image;
}
