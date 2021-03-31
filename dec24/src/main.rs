use aoc_2019_24::{a, b};
use util::io::get_input;

fn main() {
    let input = get_input();

    println!("a: {}", a::solve(&input));
    println!("b: {}", b::solve(&input));
}
