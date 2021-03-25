use aoc_2019_15::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec15.txt");

    assert_eq!(solve_a(&input), 216);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec15.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
