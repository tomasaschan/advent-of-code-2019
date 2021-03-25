use aoc_2019_11::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec11.txt");

    assert_eq!(solve_a(&input), 1909);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec11.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
