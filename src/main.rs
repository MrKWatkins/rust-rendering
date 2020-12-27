use crate::configuration::from_command_line;
use crate::rendering::algorithms::RayTracing;
use crate::rendering::{render, SubPixelSampling};
use crate::scene::io::json::load;
use std::time::Instant;

// Some modules declared as pub to suppress dead code warnings.
mod configuration;
pub mod image;
pub mod material;
pub mod maths;
pub mod rendering;
pub mod scene;

fn main() {
    let configuration = from_command_line().unwrap();

    println!("Scene: {:?}", configuration.scene);
    println!("Output file: {:?}", configuration.output);
    println!("Image size: {}x{}", configuration.width, configuration.height);

    let scene = match time_function("load scene", || load(&configuration.scene)) {
        Ok(t) => t,
        Err(e) => {
            println!("Could not load scene{:?}: {}", configuration.scene, e.to_string());
            return;
        }
    };

    let algorithm = RayTracing::new();

    let image = time_function("render", || render(&algorithm, &configuration, &scene, SubPixelSampling::Square(2)));

    let rgb = time_function("to_rgb_image", || image.to_rgb_image());

    time_function("save", || {
        rgb.save(&configuration.output)
            .unwrap_or_else(|e| println!("Could not save image: {}", e.to_string()))
    });
}

fn time_function<TResult>(name: &str, function: impl Fn() -> TResult) -> TResult {
    let start = Instant::now();

    let result = function();
    let duration = start.elapsed();

    println!("Time elapsed for {} is: {:?}", name, duration);

    return result;
}
