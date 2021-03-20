use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn get_input_no_trim() -> String {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read input from stdin!");
    input
}

pub fn get_input() -> String {
    get_input_no_trim().trim().to_string()
}

pub fn get_first_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input from stdin!");
    input.trim().to_string()
}

pub fn read_file<P: AsRef<Path>>(name: P) -> String {
    let mut input = String::new();

    let mut f = File::open(name).expect("Could not open input file");

    f.read_to_string(&mut input)
        .expect("Failed to read from input file");
    input.trim().to_string()
}
