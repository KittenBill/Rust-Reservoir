use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reservoir_in_rust::simple_reservoir::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let to_bench = ||{
        let mut sr = SimpleReservoir::new(10);

            for i in 0..100000 {
                sr.try_sample(&i);
            }

            //println!("{:?}", sr.get_sample_result());
    };
    c.bench_function(
        "SimpleReservoir 10 samples, 100000 elements in total",
        |b| b.iter(to_bench),
    );
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
