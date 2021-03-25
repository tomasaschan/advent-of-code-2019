use num::pow::Pow;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Modulo<const N: u128> {
    remainder: u128,
}
impl<const N: u128> Modulo<N> {
    pub fn new(remainder: u128) -> Modulo<N> {
        Modulo {
            remainder: remainder % N,
        }
    }
    pub fn remainder(self) -> u128 {
        self.remainder
    }
    pub fn inv(self) -> Modulo<N> {
        self.pow(N - 2)
    }
}
impl<const N: u128> std::fmt::Display for Modulo<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{} mod {}", self.remainder, N))
    }
}
impl<const N: u128> std::ops::Mul<Modulo<N>> for Modulo<N> {
    type Output = Self;
    fn mul(self, rhs: Modulo<N>) -> Self {
        Modulo {
            remainder: (self.remainder * rhs.remainder) % N,
        }
    }
}
impl<const N: u128> std::ops::Mul<Modulo<N>> for u128 {
    type Output = Modulo<N>;
    fn mul(self, rhs: Modulo<N>) -> Modulo<N> {
        Modulo {
            remainder: (rhs.remainder * self) % N,
        }
    }
}
impl<const N: u128> std::ops::Add<Modulo<N>> for Modulo<N> {
    type Output = Self;
    fn add(self, rhs: Modulo<N>) -> Self {
        Modulo {
            remainder: (self.remainder + rhs.remainder) % N,
        }
    }
}
impl<const N: u128> std::ops::Neg for Modulo<N> {
    type Output = Self;
    fn neg(self) -> Self {
        Modulo {
            remainder: N - self.remainder,
        }
    }
}
impl<const N: u128> num::pow::Pow<u128> for Modulo<N> {
    type Output = Self;
    fn pow(self, exponent: u128) -> Self {
        if N == 1 {
            Modulo::<N>::new(0)
        } else {
            let mut result = 1_u128;
            let mut exp = exponent as u128;
            let mut base = (self.remainder % N) as u128;
            let n = N as u128;
            while exp > 0 {
                if exp % 2 == 1 {
                    result = (result * base) % n;
                }
                exp = exp >> 1;
                base = (base * base) % n;
            }
            Modulo::<N>::new(result as u128 % N)
        }
    }
}

#[cfg(test)]
mod mod_tests {
    use super::*;
    use num::pow::Pow;

    #[test]
    fn mod_pow() {
        assert_eq!(Modulo::<13>::new(5).pow(3), Modulo::<13>::new(8));
        assert_eq!(Modulo::<497>::new(4).pow(13), Modulo::<497>::new(445));
    }

    #[test]
    fn mod_inv() {
        for i in (1..13).map(|i| Modulo::<13>::new(i)) {
            assert_eq!(Modulo::<13>::new(1), i * i.inv(), "failed to invert {}", i)
        }
    }
}
