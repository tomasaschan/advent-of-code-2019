use num::pow::Pow;
use util::modulo::*;

pub fn solve_a(input: &String) -> u128 {
    let deck = parse::<10007>(&input);
    deck.lookup(2019)
}

pub fn solve_b(input: &String) -> u128 {
    let deck = parse::<119315717514047>(&input).repeat(101741582076661);
    deck.get(2020)
}

#[cfg(test)]
mod solver_tests {
    use super::*;
    use util::io::read_file;

    #[test]
    fn a() {
        let input = read_file("../input/dec22.txt");
        assert_eq!(solve_a(&input), 8502);
    }

    #[test]
    fn b() {
        let input = read_file("../input/dec22.txt");
        assert_eq!(solve_b(&input), 41685581334351);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AffineTransform<const N: u128> {
    k: Modulo<N>,
    m: Modulo<N>,
}
impl<const N: u128> AffineTransform<N> {
    fn id() -> AffineTransform<N> {
        AffineTransform::<N> {
            k: Modulo::<N>::new(1),
            m: Modulo::<N>::new(0),
        }
    }
    fn new(k: u128, m: u128) -> AffineTransform<N> {
        AffineTransform::<N> {
            k: Modulo::<N>::new(k),
            m: Modulo::<N>::new(m),
        }
    }
    fn transform(self, x: Modulo<N>) -> Modulo<N> {
        self.k * x + self.m
    }
    fn invert(self) -> AffineTransform<N> {
        let kk = self.k.pow(N - 2);
        AffineTransform::<N> {
            k: kk,
            m: -(kk * self.m),
        }
    }
}

impl<const N: u128> std::ops::Add<AffineTransform<N>> for AffineTransform<N> {
    type Output = Self;
    fn add(self, rhs: AffineTransform<N>) -> AffineTransform<N> {
        AffineTransform::<N> {
            k: self.k * rhs.k,
            m: (self.k * rhs.m) + self.m,
        }
    }
}
impl<const N: u128> std::ops::AddAssign<AffineTransform<N>> for AffineTransform<N> {
    fn add_assign(&mut self, rhs: AffineTransform<N>) {
        let (k, m) = (self.k, self.m);
        self.k = k * rhs.k;
        self.m = k * rhs.m + m;
    }
}

impl<const N: u128> std::ops::Mul<u128> for AffineTransform<N> {
    type Output = Self;
    fn mul(self, rhs: u128) -> AffineTransform<N> {
        let mut result = AffineTransform::<N>::id();
        let mut n = rhs;
        let mut x = self;

        while n > 0 {
            if n % 2 == 1 {
                result += x;
            }
            n = n >> 1;
            x += x
        }
        result
    }
}
impl<const N: u128> std::ops::Mul<AffineTransform<N>> for u128 {
    type Output = AffineTransform<N>;
    fn mul(self, rhs: AffineTransform<N>) -> AffineTransform<N> {
        rhs * self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Deck<const N: u128> {
    shuffle: AffineTransform<N>,
}
impl<const N: u128> Deck<N> {
    fn new() -> Deck<N> {
        Deck {
            shuffle: AffineTransform::id(),
        }
    }

    fn stack(self) -> Self {
        Deck {
            shuffle: self.shuffle + AffineTransform::<N>::new(N - 1, N - 1),
        }
    }

    fn cut(self, i: isize) -> Deck<N> {
        let ii: u128 = if i < 0 {
            (N as isize + i) as u128
        } else {
            i as u128
        };

        Deck {
            shuffle: self.shuffle + AffineTransform::<N>::new(1, ii),
        }
    }

    fn deal(self, i: u128) -> Deck<N> {
        Deck {
            shuffle: self.shuffle
                + AffineTransform::<N> {
                    k: Modulo::<N>::new(i).inv(),
                    m: Modulo::<N>::new(0),
                },
        }
    }

    fn repeat(self, i: u128) -> Deck<N> {
        Deck {
            shuffle: i * self.shuffle,
        }
    }

    fn get(self, i: u128) -> u128 {
        self.shuffle.transform(Modulo::<N>::new(i)).remainder()
    }

    fn lookup(self, i: u128) -> u128 {
        self.shuffle
            .invert()
            .transform(Modulo::<N>::new(i))
            .remainder()
    }
}

#[cfg(test)]
mod deck_tests {
    use super::*;
    fn testit<const N: u128, const M: usize>(deck: &Deck<N>, expected: [u128; M]) {
        assert_eq!(M as u128, N);
        let mut output: [u128; M] = [0; M];
        for i in 0..M {
            output[i] = deck.get(i as u128);
        }
        assert_eq!(output, expected);
    }

    #[test]
    fn stack() {
        testit(
            &Deck::<11>::new().stack(),
            [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        );
    }

    #[test]
    fn stack_stack() {
        testit(
            &Deck::<13>::new().stack().stack(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        );
    }

    #[test]
    fn cut() {
        testit(
            &Deck::<11>::new().cut(3),
            [3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2],
        );
    }

    #[test]
    fn cut_cut() {
        testit(
            &Deck::<13>::new().cut(3).cut(5),
            [8, 9, 10, 11, 12, 0, 1, 2, 3, 4, 5, 6, 7],
        );
    }

    #[test]
    fn deal_5_deal_3() {
        testit(
            &Deck::<13>::new().deal(5).deal(3),
            [0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6],
        );
    }

    #[test]
    fn deal_5_cut_3() {
        testit(
            &Deck::<13>::new().deal(5).cut(3),
            [11, 6, 1, 9, 4, 12, 7, 2, 10, 5, 0, 8, 3],
        );
    }

    #[test]
    fn cut_3_deal_5() {
        testit(
            &Deck::<13>::new().cut(3).deal(5),
            [3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5, 0, 8],
        );
    }

    #[test]
    fn deal_5_stack() {
        testit(
            &Deck::<13>::new().deal(5).stack(),
            [5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8, 0],
        );
    }

    #[test]
    fn stack_deal_5() {
        testit(
            &Deck::<13>::new().stack().deal(5),
            [12, 4, 9, 1, 6, 11, 3, 8, 0, 5, 10, 2, 7],
        );
    }

    #[test]
    fn stack_cut_5() {
        testit(
            &Deck::<13>::new().stack().cut(5),
            [7, 6, 5, 4, 3, 2, 1, 0, 12, 11, 10, 9, 8],
        );
    }

    #[test]
    fn cut_5_stack() {
        testit(
            &Deck::<13>::new().cut(5).stack(),
            [4, 3, 2, 1, 0, 12, 11, 10, 9, 8, 7, 6, 5],
        );
    }

    #[test]
    fn deal13_1() {
        testit(
            &Deck::<13>::new().deal(1),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        );
    }

    #[test]
    fn deal13_2() {
        testit(
            &Deck::<13>::new().deal(2),
            [0, 7, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 6],
        );
    }

    #[test]
    fn deal13_3() {
        testit(
            &Deck::<13>::new().deal(3),
            [0, 9, 5, 1, 10, 6, 2, 11, 7, 3, 12, 8, 4],
        );
    }

    #[test]
    fn deal13_4() {
        testit(
            &Deck::<13>::new().deal(4),
            [0, 10, 7, 4, 1, 11, 8, 5, 2, 12, 9, 6, 3],
        );
    }

    #[test]
    fn deal13_5() {
        testit(
            &Deck::<13>::new().deal(5),
            [0, 8, 3, 11, 6, 1, 9, 4, 12, 7, 2, 10, 5],
        );
    }

    #[test]
    fn deal13_6() {
        testit(
            &Deck::<13>::new().deal(6),
            [0, 11, 9, 7, 5, 3, 1, 12, 10, 8, 6, 4, 2],
        );
    }

    #[test]
    fn deal13_7() {
        testit(
            &Deck::<13>::new().deal(7),
            [0, 2, 4, 6, 8, 10, 12, 1, 3, 5, 7, 9, 11],
        );
    }

    #[test]
    fn deal13_8() {
        testit(
            &Deck::<13>::new().deal(8),
            [0, 5, 10, 2, 7, 12, 4, 9, 1, 6, 11, 3, 8],
        );
    }

    #[test]
    fn deal13_9() {
        testit(
            &Deck::<13>::new().deal(9),
            [0, 3, 6, 9, 12, 2, 5, 8, 11, 1, 4, 7, 10],
        );
    }

    #[test]
    fn deal13_10() {
        testit(
            &Deck::<13>::new().deal(10),
            [0, 4, 8, 12, 3, 7, 11, 2, 6, 10, 1, 5, 9],
        );
    }

    #[test]
    fn deal13_11() {
        testit(
            &Deck::<13>::new().deal(11),
            [0, 6, 12, 5, 11, 4, 10, 3, 9, 2, 8, 1, 7],
        );
    }

    #[test]
    fn deal13_12() {
        testit(
            &Deck::<13>::new().deal(12),
            [0, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
    }
}

fn parse<const N: u128>(input: &String) -> Deck<N> {
    let mut deck = Deck::<N>::new();
    for line in input.lines() {
        deck = if line == "deal into new stack" {
            deck.stack()
        } else if line.starts_with("deal with increment") {
            let i = line
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .expect(&format!("failed to parse increment from line '{}'", line));
            deck.deal(i)
        } else if line.starts_with("cut") {
            let i: isize = line
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .expect(&format!("failed to parse increment from line '{}'", line));
            deck.cut(i)
        } else {
            panic!("Could not parse line '{}'", line)
        }
    }
    deck
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    #[test]
    fn parse_stack() {
        assert_eq!(
            parse::<13>(&"deal into new stack".to_string()),
            Deck::<13>::new().stack(),
        )
    }

    #[test]
    fn parse_deal() {
        assert_eq!(
            parse::<13>(&"deal with increment 3".to_string()),
            Deck::<13>::new().deal(3),
        )
    }

    #[test]
    fn parse_cut_positive() {
        assert_eq!(parse::<13>(&"cut 3".to_string()), Deck::<13>::new().cut(3));
    }

    #[test]
    fn parse_cut_negative() {
        assert_eq!(
            parse::<17>(&"cut -4".to_string()),
            Deck::<17>::new().cut(-4)
        );

        assert_eq!(
            parse::<17>(&"cut -4".to_string()),
            Deck::<17>::new().cut(13)
        );
    }
}
