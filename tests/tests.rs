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
