mod nat;
mod nic;

use nat::run_nat;

#[derive(Clone, Copy, Debug)]
pub struct Packet {
    addr: i128,
    x: i128,
    y: i128,
}

pub fn solve_a(input: &String) -> i128 {
    run_nat(&input, true)
}

pub fn solve_b(input: &String) -> i128 {
    run_nat(&input, false)
}

#[cfg(test)]
mod solver_tests {
    use super::*;
    use intcode::Builder;
    use util::io::read_file;

    #[test]
    fn test_input_hook() {
        let (input, output) = Builder::new()
            .parse(&"3,5,4,5,99".to_string())
            .input_hook(vec![104, -17])
            .run();

        assert_eq!(
            output
                .recv_timeout(std::time::Duration::from_secs(1))
                .unwrap(),
            -17
        );
        input.send(42).unwrap();
        assert_eq!(
            output
                .recv_timeout(std::time::Duration::from_secs(3))
                .unwrap(),
            42
        );
    }

    #[test]
    // #[cfg(allow_dead_code)]
    fn a() {
        let input = read_file("../input/dec23.txt");
        assert_eq!(solve_a(&input), 19473);
    }

    #[test]
    fn b() {
        let input = read_file("../input/dec23.txt");
        assert_eq!(solve_b(&input), 12475);
    }
}
