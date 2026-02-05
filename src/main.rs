use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let path = Path::new();
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Failed to read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}
