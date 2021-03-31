pub mod a {
    use super::*;
    use std::collections::HashSet;

    pub fn solve(input: &String) -> usize {
        let mut state = parse(&input);
        let mut seen = HashSet::new();
        while !seen.contains(&state) {
            seen.insert(state);
            state = step(state);
        }
        state
    }

    fn step(state: usize) -> usize {
        let mut next = 0;
        for y in 0..5 {
            for x in 0..5 {
                let k = at(x, y);
                let bm = bitmap_around(x as usize, y as usize);
                let n = (state & bm).count_ones();
                if state & k == k && n == 1 {
                    next |= k;
                } else if state & k != k && (n == 1 || n == 2) {
                    next |= k;
                }
            }
        }
        next
    }
    #[cfg(test)]
    mod step_tests {
        use super::*;
        #[test]
        fn empty_stays_empty() {
            assert_eq!(step(0), 0);
        }
        #[test]
        fn lonely_bug_dies() {
            let state = parse(&".....\n.....\n..#..\n.....\n.....".to_string());
            let next = parse(&".....\n..#..\n.#.#.\n..#..\n.....".to_string());
            assert_eq!(step(state), next);
        }
    }
}

pub mod b {
    use super::*;
    use std::collections::HashMap;

    pub fn solve(input: &String) -> u32 {
        let state = step_n(&input, 200);
        bugs(&state)
    }

    pub fn bugs(state: &State) -> u32 {
        state.values().map(|layer| layer.count_ones()).sum()
    }

    pub fn step_n(input: &String, n: usize) -> State {
        let mut state = parse(&input);

        for _ in 0..n {
            state = step(&state);
        }

        state
    }

    type State = HashMap<isize, usize>;

    pub fn parse(input: &String) -> State {
        let mut s = HashMap::new();
        s.insert(0, super::parse(&input));
        s
    }

    fn inside(i: isize) -> isize {
        i + 1
    }
    fn outside(i: isize) -> isize {
        i - 1
    }

    fn step(state: &State) -> State {
        let (lo, hi) = state.keys().fold((0, 0), |(lo, hi), i| {
            (std::cmp::min(lo, *i), std::cmp::max(hi, *i))
        });
        let mut updated = HashMap::new();

        for i in (lo - 1)..=(hi + 1) {
            let outside = state.get(&outside(i)).unwrap_or(&0);
            let inside = state.get(&inside(i)).unwrap_or(&0);
            let this = state.get(&i).unwrap_or(&0);
            let mut next = 0;

            for x in 0..5 {
                for y in 0..5 {
                    let k = at(x, y);
                    if k == CENTER {
                        continue;
                    }

                    let mut n = 0;

                    if k & LEFT != 0 {
                        n += (outside & LEFT_OF_CENTER).count_ones();
                    }
                    if k & RIGHT != 0 {
                        n += (outside & RIGHT_OF_CENTER).count_ones();
                    }
                    if k & TOP != 0 {
                        n += (outside & ABOVE_CENTER).count_ones();
                    }
                    if k & BOTTOM != 0 {
                        n += (outside & BELOW_CENTER).count_ones();
                    }
                    if k & ABOVE_CENTER != 0 {
                        n += (inside & TOP).count_ones();
                    }
                    if k & BELOW_CENTER != 0 {
                        n += (inside & BOTTOM).count_ones();
                    }
                    if k & RIGHT_OF_CENTER != 0 {
                        n += (inside & RIGHT).count_ones();
                    }
                    if k & LEFT_OF_CENTER != 0 {
                        n += (inside & LEFT).count_ones();
                    }

                    n += (bitmap_around(x, y) & this).count_ones();
                    let has_bug = (this & k) == k;
                    if has_bug && n == 1 {
                        next |= k;
                    } else if !has_bug && (n == 1 || n == 2) {
                        next |= k;
                    }
                }
            }

            updated.insert(i, next);
        }
        updated
    }

    pub fn show(state: &State) -> String {
        let (lo, hi) = state.keys().fold((0, 0), |(lo, hi), i| {
            (std::cmp::min(lo, *i), std::cmp::max(hi, *i))
        });
        let mut s = String::new();
        for i in lo..=hi {
            let lvl = state.get(&i).unwrap_or(&0);
            s += &format!("Depth: {}:\n", i).to_string();
            s += &super::show(*lvl, true);
            s += "\n";
        }

        s
    }
}

const TOP: usize = at(0, 0) | at(1, 0) | at(2, 0) | at(3, 0) | at(4, 0);
const BOTTOM: usize = at(0, 4) | at(1, 4) | at(2, 4) | at(3, 4) | at(4, 4);
const LEFT: usize = at(0, 0) | at(0, 1) | at(0, 2) | at(0, 3) | at(0, 4);
const RIGHT: usize = at(4, 0) | at(4, 1) | at(4, 2) | at(4, 3) | at(4, 4);

const ABOVE_CENTER: usize = at(2, 1);
const BELOW_CENTER: usize = at(2, 3);
const LEFT_OF_CENTER: usize = at(1, 2);
const RIGHT_OF_CENTER: usize = at(3, 2);
const CENTER: usize = at(2, 2);

fn show(lvl: usize, center_as_question_mark: bool) -> String {
    let mut s = String::new();
    for y in 0..5 {
        for x in 0..5 {
            if center_as_question_mark && at(x, y) == CENTER {
                s += "?";
            } else if (at(x, y) & lvl) != 0 {
                s += "#";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    s
}

const fn at(x: usize, y: usize) -> usize {
    1 << (y * 5 + x)
}
fn bitmap_around(x: usize, y: usize) -> usize {
    let mut bitmap = 0;
    if 0 < x {
        bitmap |= at(x - 1, y);
    }
    if x < 4 {
        bitmap |= at(x + 1, y);
    }
    if 0 < y {
        bitmap |= at(x, y - 1);
    }
    if y < 4 {
        bitmap |= at(x, y + 1);
    }
    bitmap
}

fn parse(input: &String) -> usize {
    let mut n = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => n |= at(x, y),
                '.' => {}
                '?' => {}
                _ => panic!("unexpected {} in input", c),
            }
        }
    }
    n
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    #[test]
    fn parse_example() {
        let input = ".....\n.....\n.....\n#....\n.#...".to_string();
        let parsed = parse(&input);

        assert_eq!(parsed, 2129920);
    }
}

#[cfg(test)]
mod solver_tests {
    use super::{a, b};
    use util::io::read_file;

    #[test]
    fn a() {
        let input = read_file("../input/dec24.txt");
        assert_eq!(a::solve(&input), 28615131);
    }

    #[test]
    fn b() {
        let input = read_file("../input/dec24.txt");
        assert_eq!(b::solve(&input), 0);
    }

    #[test]
    fn b_sample() {
        let input = &"....#\n#..#.\n#.?##\n..#..\n#....".to_string();
        let fin = b::step_n(input, 10);
        assert_eq!(b::bugs(&fin), 99)
    }
}
