use aoc_2019_04::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec04.txt");

    assert_eq!(solve_a(&input), 945);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec04.txt");

    assert_eq!(solve_b(&input), 617);
}
