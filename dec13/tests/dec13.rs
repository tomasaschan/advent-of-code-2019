use aoc_2019_13::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec13.txt");

    assert_eq!(solve_a(&input), 277);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec13.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
