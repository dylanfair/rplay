use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use std::{process, io, env};

pub struct Config {
    pub search_string: String,
    pub game_directories: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Arguments logic
        if args.len() < 2 {
            return Err("Need to supply a search term");
        }
        if args.len() > 2 {
            return Err("Too many arguments supplied.");
        }
        // Search string
        let search_string = args[1].clone();

        // Paths logic
        let mut steam_path = String::new();
        let mut blizzard_path = String::new();
        // PC paths
        if env::consts::OS == "windows" {
            
            let steam_env = env::var("STEAM_PATH").unwrap_or_else(|_| {
                eprintln!("No STEAM_PATH environment variable found");
                String::new()
            });
            let blizzard_env = env::var("BLIZZARD_PATH").unwrap_or_else(|_| {
                eprintln!("No BLIZZARD_PATH environment variable found");
                String::new()
            });

            steam_path.push_str(&steam_env);
            blizzard_path.push_str(&blizzard_env);
        } else { // other OS systems
            return Err("Sorry, this program only works on windows systems!")
        }
    
        // Put directories in a vector
        let mut game_directories: Vec<String> = vec![];
        if !steam_path.is_empty() {
            game_directories.push(steam_path);
        }
        if !blizzard_path.is_empty() {
            game_directories.push(blizzard_path);
        }
        
        Ok(Config { search_string, game_directories, })
    }
}

pub fn search_for_executables(search_term: &str, directories: Vec<String>) -> Result<Vec<PathBuf>, &'static str> {
    if directories.len() == 0 {
        return Err("No game directories to search, are you sure they've been set?")
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

    return Ok(vec_paths)
}

pub fn run_paths(paths: Vec<PathBuf>) -> Result<(), &'static str> {
    // Choice logic
    // No executable found
    if paths.len() < 1 {
        return Err("Could not find a matching executable.");
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
                return Ok(());
            },
            "n" => return Ok(()),
            _ => {
                return Err("Please respond with a 'y' or 'n'");
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
                return Err("Could not read your input, please try a valid number next time");
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
                    return Ok(())
                }
                else if choice == vec_length + 1 {
                    return Ok(())
                }
                else if choice > vec_length + 1 {
                    return Err("Please choose a number within the correct range.")
                } 
            }
            _ => {
                return Err("Please respond with a valid number.")
            }
        }
    }
    return Ok(())
}