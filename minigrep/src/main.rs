use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    eprintln!(
        "Searching for {} in file {}",
        config.query, config.file_name
    );

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    };
}
