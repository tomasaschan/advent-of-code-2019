use aoc_2019_18::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec18.txt");

    assert_eq!(solve_a(&input), 5808);
}
