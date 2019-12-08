use aoc_2019_05::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec05.txt");

    assert_eq!(solve_a(&input), 15314507);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec05.txt");

    assert_eq!(solve_b(&input), 652726);
}
