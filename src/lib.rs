#![no_std]

pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
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
}

macro_rules! gcd_impl {
    ($($t:ty),*) => ($(
        impl Gcd for $t {
            fn gcd(self, mut v: Self) -> Self {
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
                    v = v >> (v.trailing_zeros());
                    if u > v {
                        core::mem::swap(&mut v, &mut u);
                    }
                    v -= u; // Here v >= u.
                    if v == 0 {
                        break;
                    }
                }
                u << shift
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
        assert_eq!(0, 0u8.gcd(0));
        assert_eq!(10, 10u8.gcd(0));
        assert_eq!(10, 0u8.gcd(10));
        assert_eq!(10, 10u8.gcd(20));
        assert_eq!(44, 2024u32.gcd(748));
    }
}
