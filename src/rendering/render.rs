use crate::configuration::Configuration;
use crate::image::Image;
use crate::rendering::algorithms::Algorithm;
use crate::rendering::{RasterCoords, RasterSpace};
use crate::scene::Scene;

pub fn render<TAlgorithm: Algorithm>(algorithm: &TAlgorithm, configuration: &Configuration, scene: &Scene) -> Image {
    let raster_space = RasterSpace::new(configuration.width, configuration.height);

    let mut image = Image::new(configuration.width, configuration.height);

    for x in 0..configuration.width {
        for y in 0..configuration.height {
            let raster_coords = RasterCoords::new(x, y);

            let screen_space_coords = raster_space.to_screen_space(raster_coords);

            let colour = algorithm.render_point(scene, &screen_space_coords);

            image.set_pixel(x, y, colour);
        }
    }

    return image;
}
