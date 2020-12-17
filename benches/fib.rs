use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cargo_android_sample::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci_exp(black_box(20))));
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci_lin(black_box(20))));
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci_log(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
