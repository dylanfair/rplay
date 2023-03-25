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