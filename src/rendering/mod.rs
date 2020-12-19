use crate::configuration::Configuration;
use crate::image::Image;
use crate::maths::Scalar;
use crate::rendering::algorithms::Algorithm;
use crate::scene::Scene;
use std::cmp;

pub mod algorithms;

pub fn render<TAlgorithm: Algorithm>(algorithm: &TAlgorithm, configuration: &Configuration, scene: &Scene) -> Image {
    // Map the size of the image to -0.5 -> 0.5 on the largest side, i.e. a 1 sized box around the origin.
    let scale = cmp::max(configuration.width, configuration.height) as Scalar;

    let half_width = configuration.width as Scalar / 2.0;
    let half_height = configuration.height as Scalar / 2.0;

    let mut image = Image::new(configuration.width, configuration.height);

    for x in 0..configuration.width {
        for y in 0..configuration.height {
            let colour = algorithm.render_point((x as Scalar - half_width) / scale, (y as Scalar - half_height) / scale, scene);

            image.set_pixel(x, y, colour);
        }
    }
    return image;
}
