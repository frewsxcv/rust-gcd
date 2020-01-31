#![no_std]

pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using [`gcd_binary`].
    ///
    /// [`gcd_binary`]: #method.gcd_binary
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd::Gcd;
    ///
    /// assert_eq!(0, 0u8.gcd(0));
    /// assert_eq!(10, 10u8.gcd(0));
    /// assert_eq!(10, 0u8.gcd(10));
    /// assert_eq!(10, 10u8.gcd(20));
    /// assert_eq!(44, 2024u32.gcd(748));
    /// ```
    fn gcd(self, other: Self) -> Self;
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).
    fn gcd_binary(self, other: Self) -> Self;
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
    fn gcd_euclid(self, other: Self) -> Self;
}

macro_rules! gcd_impl {
    ($($t:ty),*) => ($(
        impl Gcd for $t {
            fn gcd(self,other: Self) -> Self {
                self.gcd_binary(other)
            }
            fn gcd_binary(self, mut v: Self) -> Self {
                let mut u = self;
                if u == 0 {
                    return v;
                }
                if v == 0 {
                    return u;
                }
                let shift = (u | v).trailing_zeros();
                u >>= shift;
                v >>= shift;
                u >>= u.trailing_zeros();
                loop {
                    v >>= v.trailing_zeros();
                    if u > v {
                        //XOR swap algorithm
                        v ^= u;
                        u ^= v;
                        v ^= u;
                    }
                    v -= u; // Here v >= u.
                    if v == 0 {
                        break;
                    }
                }
                u << shift
            }
            fn gcd_euclid(self, other: Self) -> Self {
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

gcd_impl! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod test {
    use super::Gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(0, 0u8.gcd_euclid(0));
        assert_eq!(10, 10u8.gcd_euclid(0));
        assert_eq!(10, 0u8.gcd_euclid(10));
        assert_eq!(10, 10u8.gcd_euclid(20));
        assert_eq!(44, 2024u32.gcd_euclid(748));

        assert_eq!(0, 0u8.gcd_binary(0));
        assert_eq!(10, 10u8.gcd_binary(0));
        assert_eq!(10, 0u8.gcd_binary(10));
        assert_eq!(10, 10u8.gcd_binary(20));
        assert_eq!(44, 2024u32.gcd_binary(748));
    }
}
