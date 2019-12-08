use regex::Regex;

pub fn solve_a(input: &String) -> i32 {
    let (lo, hi) = parse(&input);
    let (a, _) = solve(lo, hi);
    a
}
pub fn solve_b(input: &String) -> i32 {
    let (lo, hi) = parse(&input);
    let (_, b) = solve(lo, hi);
    b
}

fn solve(lo: i32, hi: i32) -> (i32, i32) {
    let mut a = 0;
    let mut b = 0;
    for n in lo..hi {
        let s = &n.to_string().chars().collect();
        if non_decreasing_digits(s) {
            if at_least_one_double_not_in_larger_group(s) {
                a += 1;
                b += 1;
            } else if at_least_one_double(s) {
                a += 1;
            }
        }
    }
    (a, b)
}

fn at_least_one_double(n: &Vec<char>) -> bool {
    for i in 0..n.len() - 1 {
        if n[i] == n[i + 1] {
            return true;
        }
    }
    return false;
}

fn at_least_one_double_not_in_larger_group(n: &Vec<char>) -> bool {
    for i in 0..5 {
        match i {
            0 => {
                if n[i] == n[i + 1] && n[i] != n[i + 2] {
                    return true;
                }
            }
            4 => {
                if n[i] == n[i + 1] && n[i] != n[i - 1] {
                    return true;
                }
            }
            _ => {
                if n[i] == n[i + 1] && n[i] != n[i + 2] && n[i] != n[i - 1] {
                    return true;
                }
            }
        }
    }
    return false;
}

fn non_decreasing_digits(s: &Vec<char>) -> bool {
    for i in 0..s.len() - 1 {
        if s[i + 1] < s[i] {
            return false;
        }
    }
    return true;
}

fn parse(input: &String) -> (i32, i32) {
    let r = Regex::new("(\\d{6})-(\\d{6})").expect("invalid regex");
    let m = r.captures(input).expect("no match");

    (
        m.get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("invalid lower bound"),
        m.get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("invalid upper bound"),
    )
}
