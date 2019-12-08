use aoc_2019_07::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec07.txt");

    assert_eq!(solve_a(&input), 67023);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec07.txt");

    assert_eq!(solve_b(&input), 7818398);
}
