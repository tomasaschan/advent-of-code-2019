use aoc_2019_21::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec21.txt");

    assert_eq!(solve_a(&input), 0);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec21.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
