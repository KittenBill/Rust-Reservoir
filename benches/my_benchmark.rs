use std::{thread};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reservoir_in_rust::{simple_reservoir::*, parallel_reservoir::ParallelReservoir};

//  The name of this function doesn't matter
pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    const ONE_THREAD: usize = 100_0000;
    const THREAD_COUNT: usize = 8;
    const SEQ_COUNT: usize = ONE_THREAD * THREAD_COUNT;
    const SAMPLE_COUNT: usize = 1000;

    let sequencial = || {
        let mut sr = SimpleReservoir::new(SAMPLE_COUNT);

        for i in 0..SEQ_COUNT {
            sr.try_sample(&i);
        }

        let _result = sr.get_sample_result();
    };

    let parallel = || {
        let mut pr = ParallelReservoir::new(SAMPLE_COUNT);

        let mut threads = Vec::new();

        for s in 0..THREAD_COUNT {
            let handle = pr.get_sampler_handle();
            
            let sampling_thread = thread::spawn(move || {
                let thread_start = s * ONE_THREAD;
                for i in thread_start..thread_start + ONE_THREAD {
                    handle.try_sample(&i);
                }
            });// sampling thread

            threads.push(sampling_thread);
        }
        for handle in threads {
            handle.join().unwrap();
        }

        /*
        no get_sample_result() for Parallelreservoir => benchmark time of SimpleReservoir +17%
        REASON UNKNOWN
         */
        let _result = pr.get_sample_result();
    };

    let mut group = c.benchmark_group("Reservoir Sampling");
    
    group.bench_function("PARALLEL", |b| b.iter(parallel));

    group.bench_function("SEQUENCIAL", |b| b.iter(sequencial));
    
    group.finish();
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
