use criterion::{black_box, criterion_group, criterion_main, Criterion};
use two_sets::{two_sets_greedy, two_sets_naive};

fn benchmark_greedy(c: &mut Criterion) {
    c.bench_function("greedy", |b| {
        b.iter(|| two_sets_greedy(black_box(20)))
    });
}

fn benchmark_naive(c: &mut Criterion) {
    c.bench_function("naive", |b| {
        b.iter(|| two_sets_naive(black_box(20)))
    });
}

criterion_group!(benches, benchmark_greedy, benchmark_naive);
criterion_main!(benches);