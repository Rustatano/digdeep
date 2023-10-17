use std::{fs::{self}, path::PathBuf, time::Instant};
use rayon::prelude::*;

const ROOT: &str = "C:/";
const TARGET: &str = "main.rs";
const SEARCH_IN_FILES: bool = false;
fn main() {
    let now = Instant::now();
    match fs::read_dir(ROOT) {
        Ok(_) => dig_deep(vec![PathBuf::from(ROOT)]),
        Err(e) => eprintln!("ERROR: {:?}", e),
    }
    println!("dig_deep finished in {:?}", now.elapsed());
}

fn dig_deep(dirs: Vec<PathBuf>) {
    dirs.par_iter().for_each(|dir| // ~350% boost
        if dir.is_file() {
            match dir.file_name() {
                Some(trg) => {
                    match trg.to_str() {
                        Some(value) => {
                            if value == TARGET {
                                println!("{:?} found in {:?}", value, dir);
                            }
                        },
                        None => eprintln!("OSSTRING CAN'T BE CONVERTED TO &STR"),
                    }
                },
                None => eprintln!("THE FILES DOES NOT HAVE A NAME SOMEHOW"),
            }
            
            if SEARCH_IN_FILES { // TODO -> parallel serch in files
                match fs::read_to_string(dir) {
                    Ok(file) => {
                        let mut line_count: u32 = 0;
                        for line in file.lines() {
                            line_count += 1;
                            if line.contains(&TARGET) {
                                println!("{} found in {:?}, line {}", TARGET, dir, line_count);
                            }
                        }
                    },
                    Err(_) => (),
                }
            }
            
        } else if dir == &PathBuf::from("C:/Windows") { // doesnt search in system directory
        } else {
            if let Ok(parent) = fs::read_dir(dir) {
                dig_deep(parent.map(|dir| dir.unwrap().path()).collect());
            }; 
        }
    );
}
