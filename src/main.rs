use std::process;

fn main() {
    if let Err(err) = lwsm::run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}