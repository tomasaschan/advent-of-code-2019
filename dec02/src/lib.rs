use intcode::Builder;
use itertools::Itertools;

pub fn solve_a(input: &String) -> i32 {
    let output = Builder::new()
        .parse(input)
        .init_hook(&vec![3, 1, 3, 2, 99])
        .exit_hook(&vec![4, 0, 99])
        .run_noninteractive(&mut vec![12, 2].into_iter());

    *output.last().expect("No output from program")
}

pub fn solve_b(input: &String) -> i32 {
    let run_and_read_0 = |a, b| {
        let output = Builder::new()
            .parse(input)
            .init_hook(&vec![3, 1, 3, 2, 99])
            .exit_hook(&vec![4, 0, 99])
            .run_noninteractive(&mut vec![a, b].into_iter());
        *output.last().expect("No output from program")
    };

    for (a, b) in (0..99).cartesian_product(0..99) {
        let out = run_and_read_0(a, b);
        if out == 19690720 {
            return 100 * a + b;
        }
    }
    panic!("found no solution");
}
