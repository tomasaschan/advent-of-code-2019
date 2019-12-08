use aoc_2019_01::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec01.txt");

    assert_eq!(solve_a(&input), 3437969);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec01.txt");

    assert_eq!(solve_b(&input), 5154075);
}
