extern crate criterion;
extern crate gcd;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gcd::Gcd;

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_function {
        ($name:expr, $type:ty, $func:ident) => {
            c.bench_function($name, |b| {
                b.iter(|| {
                    for i in 0..core::u8::MAX {
                        for j in 0..core::u8::MAX {
                            let (i, j) = (i as $type, j as $type);
                            black_box(i.$func(j));
                        }
                    }
                })
            });
        };
    };
    bench_function!("gcd euclid u8", u8, gcd_euclid);
    bench_function!("gcd euclid u16", u16, gcd_euclid);
    bench_function!("gcd euclid u32", u32, gcd_euclid);
    bench_function!("gcd euclid u64", u64, gcd_euclid);
    bench_function!("gcd euclid u128", u128, gcd_euclid);

    bench_function!("gcd binary u8", u8, gcd_binary);
    bench_function!("gcd binary u16", u16, gcd_binary);
    bench_function!("gcd binary u32", u32, gcd_binary);
    bench_function!("gcd binary u64", u64, gcd_binary);
    bench_function!("gcd binary u128", u128, gcd_binary);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
