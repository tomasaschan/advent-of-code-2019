use aoc_2019_16::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec16.txt");

    assert_eq!(solve_a(&input), "74369033");
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec16.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
