// use std::env;
use std::{env, process, io};

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

    // Get executable paths
    let vec_paths = play::search_for_executables(&search_string, game_directories);
    println!("{:?}", vec_paths);
    // Choice logic
    // No executable found
    if vec_paths.len() < 1 {
        println!("Could not find a matching executable.");
        process::exit(1);
    }
    // One executable found
    if vec_paths.len() == 1 {
        println!("Start the following?:");
        println!("{:?}", vec_paths[0]);
        println!("(y/n)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read response.");
        let response: &str = match response.trim().expect("Failed to read response.") {
            Ok(y_or_no) => y_or_no,
            Err(_) => {
                println!("Could not read input.");
                "Fail"
            }, 
        };

        match response {
            "y" => {
                process::Command::new(vec_paths[0]).output().expect("failed to execute process");
                println!("Executed process!");
                process::exit(1);
            },
            "n" => process::exit(1),
            _ => process::exit(1),
        }
    }
    // More than one executable found
    if vec_paths.len() > 1 {
        let vec_length = vec_paths.len();
        let mut count = 1;
        println!("Multiple executables found, which one would you like to start?");
        for path in vec_paths {
            println!("{}. {:?}", count, path);
            count += 1;
        }
        println!("{}. Cancel", vec_length + 1);
    }
    
}
