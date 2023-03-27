// use std::env;
use std::{env, process};

fn main() {
    // Read in search term
    let args: Vec<String> = env::args().collect();

    let config = play::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Get executable paths
    let vec_paths = play::search_for_executables(&config.search_string, config.game_directories).unwrap_or_else(|err| {
        eprintln!("Problem finding game library paths: {err}");
        process::exit(1);
    });

    // Run executable paths
    play::run_paths(vec_paths).unwrap_or_else(|err| {
        eprintln!("Problem during run: {err}");
        process::exit(1);
    });    
}
