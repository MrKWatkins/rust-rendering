use crate::configuration::from_command_line;
use crate::image::Image;
use std::time::Instant;

// Some modules declared as pub to suppress dead code warnings.
mod configuration;
pub mod geometry;
pub mod image;
pub mod material;
pub mod scene;

fn main() {
    let configuration = from_command_line().unwrap();

    println!("Output file: {:?}", configuration.output);
    println!("Image size: {}x{}", configuration.width, configuration.height);

    let image = time_function("Image::new", || Image::new(configuration.width, configuration.height));
    let rgb = time_function("to_rgb_image", || image.to_rgb_image());

    time_function("save", || {
        rgb.save(&configuration.output)
            .unwrap_or_else(|e| println!("Unexpected error: {}", e.to_string()))
    });
}

fn time_function<TResult>(name: &str, function: impl Fn() -> TResult) -> TResult {
    let start = Instant::now();

    let result = function();
    let duration = start.elapsed();

    println!("Time elapsed for {} is: {:?}", name, duration);

    return result;
}
