use aoc_2019_20::*;
use util::io::read_file_no_trim;

#[test]
fn solves_a() {
    let input = read_file_no_trim("../input/dec20.txt");

    assert_eq!(solve_a(&input), 496);
}

#[test]
fn solves_b() {
    let input = read_file_no_trim("../input/dec20.txt");

    assert_eq!(solve_b(&input), 5886);
}
