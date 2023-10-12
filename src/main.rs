use std::{fs::{self}, path::PathBuf};

fn main() {
    dig_deep(fs::read_dir("C:/").unwrap().map(|dir| dir.unwrap().path()).collect(), "windows");
}

fn dig_deep(dirs: Vec<PathBuf>, target: &str) {
    for dir in &dirs {
        if dir.is_file() {
            match fs::read_to_string(dir) {
                Ok(file) => {let mut line_count = 0;
                    for line in file.lines() {
                        line_count += 1;
                        if line.contains(target) {
                            println!("{} found in {:?}, line {}", target, dir, line_count);
                        }
                    }
                },
                Err(e) => eprintln!("ERROR: {}", e),
            };

        } else if dir == &PathBuf::from("C:/Windows") { // only on windows
            eprintln!("WINDOWS FILES FOUND");
        } else {
            match fs::read_dir(dir) {
                Ok(parent) => dig_deep(parent.map(|dir| dir.unwrap().path()).collect(), target),
                Err(e) => eprintln!("ERROR: {}", e),
            };
        }
    }
}
