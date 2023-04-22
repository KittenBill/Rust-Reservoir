

use criterion::{criterion_group, criterion_main, Criterion};

use reservoir_in_rust::*;

//  The name of this function doesn't matter
pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let mut group = c.benchmark_group("Reservoir Sampling");
    
    group.bench_function("PARALLEL", |b| b.iter(parallel_run));

    group.bench_function("SEQUENCIAL", |b| b.iter(sequencial_run));
    
    group.finish();
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
