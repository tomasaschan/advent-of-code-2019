use aoc_2019_15::solve_a;
use aoc_2019_15::solve_b;
use util::io::get_first_line;

fn main() {
    let input = get_first_line();

    println!("a: {}", solve_a(&input));
    println!("b: {}", solve_b(&input));
}
