use aoc_2019_14::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec14.txt");

    assert_eq!(solve_a(&input), 612880);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec14.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
