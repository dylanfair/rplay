// use std::env;
use std::{env, process};

fn main() {
    // Read in search term
    let args: Vec<String> = env::args().collect();

    let config = play::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{:?}", config.game_directories);

    // Get executable paths
    let vec_paths = play::search_for_executables(&config.search_string, config.game_directories);
    println!("{:?}", vec_paths);

    play::run_paths(vec_paths);    
}
