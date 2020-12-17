use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_exp(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_exp(n-1) + fibonacci_exp(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci_exp(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
