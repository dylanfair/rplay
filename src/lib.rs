use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use std::process;

// use glob::glob;

// pub fn find_directory(directory_name: &str) {

//     let mut entry_vecs:Vec<&str> = vec![];
//     for entry in glob("**/*.exe").expect("Errored at first glob?") {
//         let entry_PathBuf = entry.expect("Couldn't find entry");
//         let entry_string = entry_PathBuf.to_str().expect("Couldn't get a string");
//         entry_vecs.push(entry_string);
//         println!("{}", entry.expect("Errored at entry?").display());
//     }

//     println!("Looking for {}", directory_name);
// }

pub fn search_for_executables(search_term: &str, directories: Vec<&str>) -> Vec<PathBuf> {
    if directories.len() == 0 {
        println!("No game directories to search, are you sure they've been set?");
        process::exit(1);
    }
    // Now search for this file in the directories
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: false,
        require_literal_separator: false
    };

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