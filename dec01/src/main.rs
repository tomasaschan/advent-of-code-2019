extern crate util;

use aoc_2019_01::{solve_a, solve_b};
use util::io;

fn main() {
    let input = io::get_input();

    println!("a: {}", solve_a(&input));
    println!("b: {}", solve_b(&input));
}
