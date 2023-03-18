use std::thread;

#[cfg(test)]
use reservoir_in_rust::{parallel_reservoir::*, simple_reservoir::*};

#[test]
fn simple_reservoir_quick_test() {
    let mut sr = SimpleReservoir::new(10);

    for i in 0..100000 {
        sr.try_sample(&i);
    }

    println!("{:?}", sr.get_sample_result());
}

#[test]
fn parallel_reservoir_quick_test() {
    let mut pr = ParallelReservoir::new(10);

    let mut handles = Vec::new();

    for s in 0..10 {
        let handle = pr.get_handle();
        let t_handle = thread::spawn(move || {
            let thread_start = s * 1000;
            for i in thread_start..thread_start + 1000 {
                handle.lock().unwrap().try_sample(&i);
            }
        });

        handles.push(t_handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", pr.get_sample_result());
}
