use intcode::*;
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
                scaffold_map.get(&above((x, y))),
                scaffold_map.get(&below((x, y))),
                scaffold_map.get(&left_of((x, y))),
                scaffold_map.get(&right_of((x, y))),
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
    let (i, o) = Builder::new().init_hook(vec![]).parse(input).run();
    let map = map_from_input(&input);
    scaffold::display_map(&map);
    0
}
