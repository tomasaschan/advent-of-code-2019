use util::intcode::Builder;
use util::io::get_input;

use std::iter::{once, repeat};

fn main() {
    let input = get_input();
    println!("a: {:?}", solve_a(&input));
    println!("b: {:?}", solve_b(&input));
}

fn solve_a(input: &String) -> i32 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut repeat(1));
    *output.last().unwrap()
}

fn solve_b(input: &String) -> i32 {
    let output = Builder::new()
        .parse(&input)
        .run_noninteractive(&mut once(5));
    *output.last().unwrap()
}
