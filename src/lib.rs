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
    ($($T:ty),*) => {$(

        paste::paste! {
            #[doc = "Const binary GCD implementation for `" $T "`."]
            pub const fn [<binary_ $T>](mut u: $T, mut v: $T) -> $T
            {
                if u == 0 { return v; }
                if v == 0 { return u; }

                let shift = (u | v).trailing_zeros();
                u >>= shift;
                v >>= shift;
                u >>= u.trailing_zeros();

                loop {
                    v >>= v.trailing_zeros();

                    if u > v {
                        // mem::swap(&mut u, &mut v);
                        let temp = u;
                        u = v;
                        v = temp;
                    }

                    v -= u; // here v >= u

                    if v == 0 { break; }
                }

                u << shift
            }

            #[doc = "Const euclid GCD implementation for `" $T "`."]
            pub const fn [<euclid_ $T>]( a: $T,  b: $T) -> $T
            {
                // variable names based off euclidean division equation: a = b · q + r
                let (mut a, mut b) = if a > b {
                    (a, b)
                } else {
                    (b, a)
                };

                while b != 0 {
                    // mem::swap(&mut a, &mut b);
                    let temp = a;
                    a = b;
                    b = temp;

                    b %= a;
                }

                a
            }
        }


        impl Gcd for $T {
            fn gcd(self, other: $T) -> $T {
                self.gcd_binary(other)
            }

            fn gcd_binary(self, v: $T) -> $T {
                paste::paste! {
                    [<binary_ $T>](self, v)
                }
            }

            fn gcd_euclid(self, other: $T) -> $T {
                paste::paste! {
                    [<euclid_ $T>](self, other)
                }
            }
        }
    )*};
}

gcd_impl! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod test {
    use super::*;

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

    const GCD_10_20: u16 = binary_u16(10, 20);
    const GCD_2024_748: u32 = binary_u32(2024, 44);

    #[test]
    fn test_const_gcd() {
        assert_eq!(10, GCD_10_20);
        assert_eq!(44, GCD_2024_748);
    }
}
