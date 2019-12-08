use aoc_2019_08::{solve_a, solve_b};
use util::io::get_input;

fn main() {
    let input = get_input();

    println!("a: {}", solve_a(&input));
    println!("b: {}", solve_b(&input));
}
