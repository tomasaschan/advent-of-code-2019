use aoc_2019_09::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec09.txt");

    assert_eq!(solve_a(&input), 4261108180);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec09.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
