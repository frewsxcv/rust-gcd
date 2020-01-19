extern crate criterion;
extern crate gcd;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gcd::Gcd;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gcd u8", |b| b.iter(|| {
        black_box(0u8.gcd(0));
        black_box(0u8.gcd(237));
        black_box(237u8.gcd(0));
        black_box(237u8.gcd(178));
    }));

    c.bench_function("gcd u16", |b| b.iter(|| {
        black_box(0u16.gcd(0));
        black_box(0u16.gcd(10));
        black_box(237u16.gcd(0));
        black_box(237u16.gcd(178));
    }));

    c.bench_function("gcd u32", |b| b.iter(|| {
        black_box(0u32.gcd(0));
        black_box(0u32.gcd(10));
        black_box(237u32.gcd(0));
        black_box(237u32.gcd(178));
    }));

    c.bench_function("gcd u64", |b| b.iter(|| {
        black_box(0u64.gcd(0));
        black_box(0u64.gcd(10));
        black_box(237u64.gcd(0));
        black_box(237u64.gcd(178));
    }));

    c.bench_function("gcd u128", |b| b.iter(|| {
        black_box(0u128.gcd(0));
        black_box(0u128.gcd(10));
        black_box(237u128.gcd(0));
        black_box(237u128.gcd(178));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
