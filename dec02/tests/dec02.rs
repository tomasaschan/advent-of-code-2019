use aoc_2019_02::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec02.txt");

    assert_eq!(solve_a(&input), 6327510);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec02.txt");

    assert_eq!(solve_b(&input), 4112);
}
