use crate::configuration::from_command_line;
extern crate nalgebra_glm as glm;

mod colour;
pub use colour::*;
mod configuration;

fn main() {
    let configuration = from_command_line().unwrap();

    println!("Output file: {:?}", configuration.output);
}
