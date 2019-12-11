use intcode::Builder;

pub fn solve_a(input: &String) -> i128 {
    let output = Builder::new()
        .parse(input)
        .run_noninteractive(&mut (vec![1]).into_iter());
    *output.last().expect("No output!")
}

pub fn solve_b(input: &String) -> i128 {
    let output = Builder::new()
        .parse(input)
        .run_noninteractive(&mut (vec![2]).into_iter());
    *output.last().expect("No output!")
}
