use reservoir_in_rust::{parallel_reservoir::*, simple_reservoir::*};
use std::{thread, cell::RefCell};

#[cfg(test)]
#[test]
pub fn simple_reservoir_quick_test() {
    let mut sr = SimpleReservoir::new(1000);

    for i in 0..400_0000 {
        sr.try_sample(&i);
    }

    let result = sr.get_sample_result().unwrap();
    
    println!("{:?}", result);
    
    let mut total: u128 = 0;
    for x in result.samples.iter() {
        total += *x as u128;
    }

    println!("AVG = {}", total / (result.samples.len() as u128));
}

#[test]
pub fn parallel_reservoir_quick_test() {
    let mut pr = ParallelReservoir::new(1000);

    let mut handles = Vec::new();

    for s in 0..4 {
        let handle = pr.get_sampler_handle();
        const ONE_THREAD: i32 = 100_0000;
        let t_handle = thread::spawn(move || {
            let thread_start = s * ONE_THREAD;
            for i in thread_start..thread_start + ONE_THREAD {
                handle.lock().unwrap().try_sample(&i);
            }
        });

        handles.push(t_handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let result = pr.get_sample_result().unwrap();

    println!("{:?}", result);

    let mut total: u128 = 0;
    for x in result.samples.iter() {
        total += *x as u128;
    }

    println!("AVG = {}", total / (result.samples.len() as u128));
}

const RANGE: usize = 1_0000;
const SAMPLE_COUNT: usize = 100;
const TEST_COUNT: usize = 1_0000;

#[test]
pub fn simple_reservoir_validation() -> () {
    let mut v: Vec<usize> = vec![0; RANGE];

    for _ in 0..TEST_COUNT {
        let mut sr = SimpleReservoir::new(SAMPLE_COUNT);
        for x in 0..RANGE {
            sr.try_sample(&x);
        }
        for x in sr.get_sample_result().unwrap().samples {
            v[x] += 1;
        }
    }

    println!("{:?}", v);
}

const THREAD_COUNT: usize = 4;

#[test]
pub fn parallel_reservoir_validation() -> () {
    let mut v: Vec<usize> = vec![0; RANGE];

    for _ in 0..TEST_COUNT {
        let mut pr = ParallelReservoir::new(SAMPLE_COUNT);

        const STEP:usize = RANGE / THREAD_COUNT;
        let mut sampler_threads = Vec::with_capacity(THREAD_COUNT);
        for t_idx in 0..THREAD_COUNT {
            let sampler = pr.get_sampler_handle();
            
            let sampler_thread = thread::spawn(move || {
                for x in t_idx*STEP..(t_idx+1)*STEP{
                    sampler.lock().unwrap().try_sample(&x);
                }
            });
            sampler_threads.push(sampler_thread);
        }

        for thread in sampler_threads.into_iter(){
            thread.join().unwrap();
        }

        for x in pr.get_sample_result().unwrap().samples {
            v[x] += 1;
        }
    }

    println!("{:?}", v);
}
