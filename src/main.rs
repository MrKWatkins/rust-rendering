use crate::configuration::from_command_line;

mod configuration;

fn main() {
    let configuration = from_command_line().unwrap();

    println!("Output file: {:?}", configuration.output);
}
