use std::iter::*;

pub fn solve_a(input: &String) -> String {
    let mut data: Vec<i128> = input
        .split("")
        .filter(|x| *x != "")
        .map(|c| c.parse().unwrap())
        .collect();

    data = one_hundred_phases(data);
    get_8(&data, 0)
}

pub fn solve_b(input: &String) -> String {
    let mut data: Vec<i32> = input
        .split("")
        .filter(|x| *x != "")
        .map(|c| c.parse().unwrap())
        .collect();

    let offset: usize = input
        .split("")
        .filter(|x| *x != "")
        .take(7)
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    assert_eq!(offset > (data.len() * 10_000) / 2, true);

    data = data.repeat(10_000).into_iter().skip(offset).collect();

    for _ in 0..100 {
        data = cheat(data);
    }

    data.iter()
        .take(8)
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn cheat(data: Vec<i32>) -> Vec<i32> {
    let mut n: i32 = data.iter().sum();

    data.iter()
        .rev()
        .map(|i| {
            let x = n % 10;
            n -= i;
            x
        })
        .rev()
        .collect()
}

fn one_hundred_phases(data: Vec<i128>) -> Vec<i128> {
    let mut d = data.clone();
    let phases = 100;
    for _ in 0..phases {
        d = dumb_fft(&d);
    }
    d
}

fn get_8(data: &Vec<i128>, ix: usize) -> String {
    data[ix..=ix + 7]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn dumb_fft<'a>(input: &'a Vec<i128>) -> Vec<i128> {
    (0..input.len())
        .map(|ix| dumb_fft_1(input.iter(), ix))
        .collect()
}

fn dumb_fft_1<'a, I>(input: I, ix: usize) -> i128
where
    I: Iterator<Item = &'a i128>,
{
    let sum: i128 = input.zip(pattern(ix)).map(|(s, p)| s * p).sum();
    let result = (sum % 10).abs();

    result
}

fn pattern(ix: usize) -> impl Iterator<Item = i128> {
    let items = [0, 1, 0, -1];
    let mut repeats = 0;
    let mut current = 0;

    from_fn(move || {
        repeats += 1;
        if repeats > ix {
            current += 1;
            repeats = 0;
        }

        Some(items[current % 4])
    })
}
