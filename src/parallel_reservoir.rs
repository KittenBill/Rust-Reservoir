#[allow(dead_code)]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use rand::{prelude::*, rngs::StdRng};

use super::simple_reservoir::*;

pub struct ParallelReservoir<T: Clone + Sync + Send> {
    sample_count: usize,

    sampler_handles: Vec<Arc<Mutex<SimpleReservoir<T>>>>,
}

impl<T> ParallelReservoir<T>
where
    T: Clone + Sync + Send + 'static, // todo: fix the lifetime bounds here
{
    pub fn new(sample_count: usize) -> Self {
        Self {
            sample_count,
            sampler_handles: Vec::new(),
        }
    }

    pub fn get_handle(&mut self) -> Arc<Mutex<SimpleReservoir<T>>> {
        let handle = Arc::new(Mutex::new(SimpleReservoir::new(self.sample_count)));
        self.sampler_handles.push(Arc::clone(&handle));
        handle
    }

    pub fn get_sample_result(&self) -> SampleResult<T> {
        let (tx, rx) = mpsc::channel();
        // this channel is used as a blocking queue

        for handle in self.sampler_handles.iter() {
            tx.send(
                handle
                    .lock() //
                    .unwrap()
                    .get_sample_result(),
            )
            .unwrap();
        }

        let thread_count = self.sampler_handles.len();

        let mut merger_handles = Vec::new();

        for _ in 0..thread_count - 1 {
            let mut result1 = rx.recv().unwrap();
            let mut result2 = rx.recv().unwrap();

            let tx_for_merger = tx.clone();

            let sample_count = self.sample_count;

            let handle = thread::spawn(move || {
                let mut rng = StdRng::from_entropy();

                let possibility = (result1.total as f64) / (result1.total + result2.total) as f64;

                let mut ret: Vec<T> = Vec::with_capacity(sample_count);

                let mut take_randomly = |v: &mut Vec<T>| {
                    let mut rng = thread_rng();
                    let idx = rng.gen_range(0_usize..v.len());
                    let take = v.swap_remove(idx);
                    ret.push(take);
                };

                for _ in 0..sample_count {
                    if rng.gen_range(0_f64..1_f64) < possibility {
                        take_randomly(&mut result1.samples);
                    } else {
                        take_randomly(&mut result2.samples);
                    }
                }

                tx_for_merger
                    .send(SampleResult::new(ret, result1.total + result2.total))
                    .unwrap();
            }); // merger thread

            merger_handles.push(handle);
        }

        for handle in merger_handles {
            handle.join().unwrap();
        }

        rx.recv().unwrap()
    }
}

/* pub struct SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    sampler: Mutex<SimpleReservoir<T>>,
}

impl<T> SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    pub fn new(sample_count: usize) -> Self {
        Self {
            sampler: Mutex::new(SimpleReservoir::new(sample_count)),
        }
    }
}

impl<T> Sampler<T> for SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    fn get_sample_result(&self) -> SampleResult<T> {
        self.sampler
            .lock() // lock
            .unwrap()
            .get_sample_result()
    }

    fn try_sample(&mut self, element: &T) -> bool {
        self.sampler
            .lock() // lock
            .unwrap()
            .try_sample(element)
    }
}
 */
