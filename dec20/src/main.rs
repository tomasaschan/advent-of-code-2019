use aoc_2019_20::{solve_a, solve_b};
use util::io::get_input_no_trim;

fn main() {
    let input = get_input_no_trim();

    println!("a: {}", solve_a(&input));
    println!("b: {}", solve_b(&input));
}
