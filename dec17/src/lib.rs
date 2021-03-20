use intcode::*;
// use std::iter::FromIterator;
use util::map_2d::*;

pub mod interactive;
pub mod scaffold;

pub fn solve_a(input: &String) -> i32 {
    let mut sum = 0;
    let mut scaffold_map = map_from_input(&input);

    let ((xlo, ylo), (xhi, yhi)) = scaffold_map.corners();
    for y in ylo..yhi + 1 {
        for x in xlo..xhi + 1 {
            match (
                scaffold_map.get(&(x, y)),
                scaffold_map.get(&above(&(x, y))),
                scaffold_map.get(&below(&(x, y))),
                scaffold_map.get(&left_of(&(x, y))),
                scaffold_map.get(&right_of(&(x, y))),
            ) {
                (Some('#'), Some('#'), Some('#'), Some('#'), Some('#')) => {
                    sum += x * y;
                    scaffold_map.update((x, y), |_| 'O');
                }
                _ => (),
            }
        }
    }

    sum
}

fn map_from_input(input: &String) -> WorldMap<char> {
    let output = AsciiBuilder::new()
        .parse(input)
        .run_noninteractive(&mut vec![].into_iter())
        .into_iter()
        .collect();
    scaffold::get_map(&output)
}

pub fn solve_b(input: &String) -> i128 {
    let (i, o) = Builder::new()
        .init_hook(vec![3, 0])
        .initial_input(vec![2])
        .parse(input)
        .run();

    let main_routine = "A,B,A,C,B,A,C,B,A,C";
    let a = "L,6,L,4,R,12";
    let b = "L,6,R,12,R,12,L,8";
    let c = "L,6,L,10,L,10,L,6";

    let continuous_feed = "n";
    for input_digit in (vec![main_routine, a, b, c, continuous_feed].join("\n") + "\n")
        .chars()
        .map(|c| c as u32 as i128)
    {
        i.send(input_digit).unwrap();
    }

    let output: Vec<i128> = o.iter().collect();
    // print!(
    //     "{}",
    //     String::from_iter(output.iter().map(|i| match std::char::from_u32(*i as u32) {
    //         Some(c) => c,
    //         None => ' ',
    //     }))
    // );

    match output.into_iter().last() {
        Some(i) => i,
        None => -1,
    }
}
