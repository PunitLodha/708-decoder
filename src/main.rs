use cea_708_decoder::run;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
