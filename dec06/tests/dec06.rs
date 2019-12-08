use aoc_2019_06::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec06.txt");

    assert_eq!(solve_a(&input), 295936);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec06.txt");

    assert_eq!(solve_b(&input), 457);
}
