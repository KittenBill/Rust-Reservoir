pub mod parallel_reservoir;
pub mod simple_reservoir;

use crate::simple_reservoir::*;
use crate::parallel_reservoir::*;
use std::thread;

const ONE_THREAD: usize = 100_0000;
const THREAD_COUNT: usize = 8;
const SEQ_COUNT: usize = ONE_THREAD * THREAD_COUNT;
const SAMPLE_COUNT: usize = 1000;

pub fn sequencial_run() {
    let mut sr = SimpleReservoir::new(SAMPLE_COUNT);

    for i in 0..SEQ_COUNT {
        sr.try_sample(&i);
    }

    let _result = sr.get_sample_result();
}

pub fn parallel_run(){
    let mut pr = ParallelReservoir::new(SAMPLE_COUNT);

    let mut threads = Vec::new();

    for s in 0..THREAD_COUNT {
        let handle = pr.get_sampler_handle();
        
        let sampling_thread = thread::spawn(move || {
            let thread_start = s * ONE_THREAD;
            for i in thread_start..thread_start + ONE_THREAD {
                handle.lock().unwrap().try_sample(&i);
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
}