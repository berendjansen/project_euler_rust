pub mod primes {
    use num_traits::{PrimInt, Unsigned};

    pub fn is_prime<T>(n: T) -> bool
    where
        T: PrimInt + Unsigned,
    {
        if n <= T::one() {
            return false;
        } else if n == T::from(2).unwrap() || n == T::from(3).unwrap() {
            return true;
        } else if n % T::from(2).unwrap() == T::zero() {
            return false;
        };

        let mut i = T::from(5).unwrap();
        while i * i < n + T::one() {
            if n % i == T::zero() || n % (i + T::from(2).unwrap()) == T::zero() {
                return false;
            }
            i = i + T::from(6).unwrap();
        }
        true
    }

    pub fn find_nth_prime(n: usize) -> u32 {
        if n <= 2 {
            return [2, 3][n - 1];
        };

        let mut i = 5u32;
        let mut c = 2;

        while c < n {
            if is_prime(i) {
                c += 1;
                if c == n {
                    return i;
                }
            }
            if is_prime(i + 2) {
                c += 1;
                if c == n {
                    return i + 2;
                }
            }
            i += 6;
        }
        unreachable!("Should have returned a prime number")
    }

    pub fn find_primes_below(n: usize) -> Vec<usize> {
        if n < 2 {
            return vec![];
        }

        if n <= 5 {
            return vec![2, 3];
        };

        let mut out: Vec<usize> = vec![2, 3];

        let mut i = 5usize;
        while i < n {
            if is_prime(i) {
                out.push(i)
            }
            if is_prime(i + 2) {
                out.push(i + 2)
            }
            i += 6;
        }
        out
    }
}

pub mod prime_factorisation {
    #[derive(Debug, PartialEq)]
    pub struct Factorisation {
        pub factors: Vec<Factor>,
    }

    impl Factorisation {
        pub fn new(factors: Vec<Factor>) -> Self {
            Self { factors }
        }

        pub fn max_factor(&self) -> &Factor {
            match self.factors.iter().max_by(|x, y| x.value.cmp(&y.value)) {
                Some(f) => f,
                None => panic!("No max factor found"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Factor {
        pub value: u32,
        pub power: u32,
    }

    impl Factor {
        pub fn new(value: u32, power: u32) -> Self {
            Self { value, power }
        }

        pub fn result(&self) -> usize {
            self.value.pow(self.power) as usize
        }
    }

    pub fn prime_factorisation(n: u32) -> Factorisation {
        if n < 2 {
            panic!(
                "Input for prime factorisation must be larger than 1, got {}",
                n
            );
        }

        let mut res: u32 = n;
        let mut div: u32 = 2;
        let mut output: Vec<Factor> = Vec::new();

        let mut power: u32 = 0;
        while res > 1 {
            while res % div == 0 {
                res /= div;
                power += 1;
            }
            if power > 0 {
                output.push(Factor { value: div, power });
                power = 0;
            };
            div = if div == 2 { div + 1 } else { div + 2 };
        }
        Factorisation { factors: output }
    }
}

#[cfg(test)]
mod tests {
    use super::prime_factorisation::{prime_factorisation, Factor, Factorisation};
    use super::primes::{find_nth_prime, find_primes_below, is_prime};

    #[test]
    fn test_struct() {
        let f = Factor::new(5, 3);
        assert_eq!(f.result(), 125)
    }

    #[test]
    fn test_factorisation() {
        let num: u32 = 60;
        let factors = prime_factorisation(num);
        assert_eq!(
            Factorisation::new(vec![
                Factor::new(2, 2),
                Factor::new(3, 1),
                Factor::new(5, 1)
            ]),
            factors
        );

        assert_eq!(factors.max_factor(), &Factor::new(5, 1));

        let num: u32 = 2;
        let factors = prime_factorisation(num);
        assert_eq!(Factorisation::new(vec![Factor::new(2, 1)]), factors)
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(11u32));
        assert!(is_prime(3u64));
        assert!(!is_prime(0u8));
        assert!(is_prime(7u8));
        assert!(!is_prime(1usize));
        assert!(is_prime(23u64));
    }

    #[test]
    fn test_find_nth_prime() {
        assert_eq!(find_nth_prime(1), 2);
        assert_eq!(find_nth_prime(2), 3);
        assert_eq!(find_nth_prime(3), 5);
        assert_eq!(find_nth_prime(6), 13);
    }

    #[test]
    #[should_panic]
    fn test_panic_find_nth_prime() {
        assert_eq!(find_nth_prime(0), 13);
    }

    #[test]
    fn test_find_primes_below() {
        assert_eq!(find_primes_below(5), vec![2, 3]);
        assert_eq!(find_primes_below(15), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_below(10), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_below(15), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(find_primes_below(1), vec![]);
    }
}
