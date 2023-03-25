// use std::env;
use glob::{glob_with, MatchOptions};
use std::env;


fn main() {
    // Read in search term
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return println!("Need to supply a search term");
    }
    if args.len() > 2 {
        return println!("Too many arguments supplied.");
    }

    let search_string = args[1].clone();
    
    // Linux paths
    let steam_path = "/home/dylan/Documents/play-test/SteamLibrary";
    let blizzard_path = "/home/dylan/Documents/play-test/BlizzardLibrary";
    // PC paths
    let _steam_path_pc = "";
    let _blizzard_path_pc = "";

    // Put directories in a vector
    let mut game_directories: Vec<&str> = vec![];
    game_directories.push(steam_path);
    game_directories.push(blizzard_path);

    // Now search for this file in the directories
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: false,
        require_literal_separator: false
    };

    for directory in game_directories {
        let search_string = format!("{directory}/**/*{search_string}*.exe");
        for entry in glob_with(&search_string, options).unwrap() {
            if let Ok(path) = entry {
                println!("{:?}", path.display())
            }
        }    
    }
    

    
}
