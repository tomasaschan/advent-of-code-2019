use aoc_2019_17::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec17.txt");

    assert_eq!(solve_a(&input), 4864);
}

//#[test]
//fn solves_b() {
//    let input = read_file("../input/dec17.txt");
//
//    assert_eq!(solve_b(&input), ...);
//}
