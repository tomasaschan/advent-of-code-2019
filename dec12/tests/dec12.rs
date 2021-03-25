use aoc_2019_12::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec12.txt");

    assert_eq!(solve_a(&input), 8454);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec12.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
