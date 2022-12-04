use std::{env, process};
use day04::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("failed to parse file_path: {err}");
        process::exit(1);
    });

    if let Err(e) = day04::run(config) {
        println!("application error {e}");
        process::exit(1);
    }
}
