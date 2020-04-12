use std::env;
use std::process;

use maze::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Process aborted with message:\n{}", err);
        process::exit(1);
    });

    maze::run(config)
}
