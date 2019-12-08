use aoc_2019_03::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec03.txt");

    assert_eq!(solve_a(&input), 403);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec03.txt");

    assert_eq!(solve_b(&input), 4158);
}
