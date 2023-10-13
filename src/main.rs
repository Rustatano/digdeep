use std::{fs::{self}, path::PathBuf, time::Instant};

pub const ROOT: &str = "..\\dig_deep";
pub const TARGET: &str = "main.rs";
pub const SEARCH_IN_FILES: bool = false;

fn main() {
    let now = Instant::now();
    match fs::read_dir(ROOT) {
        Ok(root) => dig_deep(root.map(|dir| dir.unwrap().path()).collect()),
        Err(_) => eprintln!("DIRECTORY NOT FOUND"),
    };
    println!("dig_deep done in {:?}", now.elapsed());
}

fn dig_deep(dirs: Vec<PathBuf>) {
    for dir in &dirs {
        if dir.is_file() {
            if let Some(trg) = dir.file_name() {
                if trg == TARGET {
                    println!("{:?} found in {:?}", TARGET, dir);
                }
            }
            if SEARCH_IN_FILES {
                match fs::read_to_string(dir) {
                    Ok(file) => {
                        let mut line_count = 0;
                        for line in file.lines() {
                            line_count += 1;
                            if line.contains(TARGET) {
                                println!("{} found in {:?}, line {}", TARGET, dir, line_count);
                            }
                        }
                    },
                    Err(e) => eprintln!("ERROR: {}", e),
                }
            }

        } else if dir == &PathBuf::from("C:/Windows") { // only on windows
            eprintln!("WINDOWS FILES FOUND");

        } else {
            match fs::read_dir(dir) {
                Ok(parent) => dig_deep(parent.map(|dir| dir.unwrap().path()).collect()),
                Err(e) => eprintln!("ERROR: {}", e),
            }; 
        }
    }
}
