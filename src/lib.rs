use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::{process, io, env};

pub struct Config {
    pub search_string: String,
    pub game_directories: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Need to supply a search term");
        }
        if args.len() > 2 {
            return Err("Too many arguments supplied.");
        }
    
        let search_string = args[1].clone();


        // let mut game_directories_env: Vec<String> = vec![];

        // let env_steam_path = env::var("steam_path").unwrap();
        // let env_blizzard_path = env::var("blizzard_path").unwrap();
    
        // game_directories_env.push(env_steam_path);
        // game_directories_env.push(env_blizzard_path);
    
        let steam_path = String::from_str("/home/dylan/Documents/play-test/SteamLibrary").unwrap();
        let blizzard_path = String::from_str("/home/dylan/Documents/play-test/BlizzardLibrary").unwrap();
        // PC paths
        let _steam_path_pc = "";
        let _blizzard_path_pc = "";
    
        // Put directories in a vector
        let mut game_directories: Vec<String> = vec![];
        game_directories.push(steam_path);
        game_directories.push(blizzard_path);


        Ok(Config { search_string, game_directories, })
    }
}

pub fn search_for_executables(search_term: &str, directories: Vec<String>) -> Vec<PathBuf> {
    if directories.len() == 0 {
        println!("No game directories to search, are you sure they've been set?");
        process::exit(1);
    }
   
    // Options we can set for our search
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: false,
        require_literal_separator: false
    };
    // Now search for this file in the directories
    let mut vec_paths:Vec<PathBuf> = vec![];
    for directory in directories {
        let search_string = format!("{directory}/**/*{search_term}*.exe");
        for entry in glob_with(&search_string, options).unwrap() {
            if let Ok(path) = entry {
                vec_paths.push(path);
            }
        }    
    }

    vec_paths
}

pub fn run_paths(paths: Vec<PathBuf>) {
    // Choice logic
    // No executable found
    if paths.len() < 1 {
        println!("Could not find a matching executable.");
        process::exit(1);
    }


    let mut response = String::new();
    // One executable found
    if paths.len() == 1 {
        // Print opening choice
        println!("Start the following?:");
        println!("{:?}", paths[0]);
        println!("(y/n)");
        // Read in user response
        io::stdin().read_line(&mut response).expect("Failed to read response.");
        // match response to an outcome
        match response.trim() {
            "y" => {
                let exe_path = paths[0].to_str().expect("Failed to stringify path");
                process::Command::new(exe_path).output().expect("failed to execute process");
                println!("Executed process!");
                process::exit(1);
            },
            "n" => process::exit(1),
            _ => {
                println!("Please respond with a 'y' or 'n'");
                process::exit(1);
            },
        };
    }
    // More than one executable found
    if paths.len() > 1 {
        // Print opening choices
        let vec_length = paths.len();
        let mut count = 1;
        println!("Multiple executables found, which one would you like to start?");
        for path in &paths {
            println!("{}. {:?}", count, path);
            count += 1;
        }
        println!("{}. Cancel", vec_length + 1);
        // Read in user response
        io::stdin().read_line(&mut response).expect("Failed to read response.");
        let choice: usize = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not read your input, please try a valid number next time");
                process::exit(1);
            },
        };


        // Match response to an outcome
        match choice > 0 {
            true => {
                if choice > 0 && choice < vec_length + 1 {
                    let path_slice = choice - 1;
                    let exe_path = paths[path_slice].to_str().expect("Failed to read number.");
                    process::Command::new(exe_path).output().expect("failed to execute process");
                    println!("Executed process!");
                    process::exit(1);
                }
                if choice == vec_length + 1 {
                    process::exit(1)
                }
                if choice > vec_length + 1 {
                    println!("Please choose a number within the correct range.");
                    process::exit(1)
                }
            }
            _ => {
                println!("Please respond with a valid number.");
                process::exit(1);
            }
        }

    }
}