use aoc_2019_08::*;
use util::io::read_file;

#[test]
fn solves_a() {
    let input = read_file("../input/dec08.txt");

    assert_eq!(solve_a(&input), 1330);
}

#[test]
fn solves_b() {
    let input = read_file("../input/dec08.txt");

    assert_eq!(
        solve_b(&input),
        "
XXXX  XX  X  X XXXX XXXX 
X    X  X X  X X    X    
XXX  X  X XXXX XXX  XXX  
X    XXXX X  X X    X    
X    X  X X  X X    X    
X    X  X X  X XXXX X    "
    );
}
