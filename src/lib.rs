pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
    fn gcd(self, other: Self) -> Self;
}

macro_rules! gcd_impl {
    ($($t:ty),*) => ($(
        impl Gcd for $t {
            fn gcd(self, other: Self) -> Self {
                // variable names based off Euclidean divison equation: a = b · q + r
                let (mut a, mut b) = if self > other {
                    (self, other)
                } else {
                    (other, self)
                };

                while b != 0 {
                    let r = a % b;
                    a = b;
                    b = r;
                }

                a
            }
        }
    )*)
}

gcd_impl! { u8, u16, u32, u64 }

#[cfg(test)]
mod test {
    use super::Gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(0, 0u8.gcd(0));
        assert_eq!(10, 10u8.gcd(0));
        assert_eq!(10, 0u8.gcd(10));
        assert_eq!(10, 10u8.gcd(20));
        assert_eq!(44, 2024u32.gcd(748));
    }
}
