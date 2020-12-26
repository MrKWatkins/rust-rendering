use crate::configuration::Configuration;
use crate::image::{Colour, Image};
use crate::maths::Coordinates;
use crate::rendering::algorithms::Algorithm;
use crate::rendering::{RasterSpace, SubPixelSampling};
use crate::scene::Scene;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;

pub fn render<TAlgorithm: Algorithm + Sync>(algorithm: &TAlgorithm, configuration: &Configuration, scene: &Scene, sampling: SubPixelSampling) -> Image {
    let image = Arc::new(Mutex::new(Image::new(configuration.width, configuration.height)));

    let raster_space = RasterSpace::new(configuration.width, configuration.height);

    let sampling_offsets = sampling.pixel_offsets();

    (0..configuration.width * configuration.height).into_par_iter().for_each_init(
        || sampling_offsets.iter().map(|_| Colour::black()).collect::<Vec<Colour>>(),
        |samples, pixel| {
            let y = pixel / configuration.width;
            let x = pixel - y * configuration.width;

            for sample in 0..sampling_offsets.len() {
                let offset = &sampling_offsets[sample];

                let raster_coords = Coordinates::new(x as f32 + offset.x, y as f32 + offset.y);

                let screen_space_coords = raster_space.to_screen_space(raster_coords);

                samples[sample] = algorithm.render_point(scene, &screen_space_coords);
            }
            image.lock().set_pixel(x, y, Colour::average(&samples));
        },
    );

    return image.lock().clone();
}
