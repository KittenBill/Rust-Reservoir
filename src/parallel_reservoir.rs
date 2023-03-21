#[allow(dead_code)]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use rand::{prelude::*, rngs::StdRng};

use super::simple_reservoir::*;

pub struct ParallelReservoir<T: Clone + Sync + Send> {
    sample_count: usize,

    sampler_handles: Vec<Arc<SamplerHandle<T>>>,
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

    pub fn get_handle(&mut self) -> Arc<SamplerHandle<T>> {
        let handle = Arc::new(SamplerHandle::new(self.sample_count));
        self.sampler_handles.push(Arc::clone(&handle));
        handle
    }

    pub fn get_sample_result(&self) -> SampleResult<T> {
        let (tx, rx) = mpsc::channel();
        // this channel is used as a blocking queue

        for handle in self.sampler_handles.iter() {
            tx.send(handle.get_sample_result()).unwrap();
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

pub struct SamplerHandle<T>
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


/**
 * 本想给SamplerHandle实现Sampler特征，
 * 但Sampler特征中的try_sample()需要的是self的可变借用，但在Arc不允许包裹的内容被可变借用
 * 
 * 一种解决方案是将Sampler中的方法改为不可变借用，再在SimpleReservoir的实现中使用RefCell以得到内部可变性
 * https://course.rs/advance/smart-pointer/cell-refcell.html#%E5%86%85%E9%83%A8%E5%8F%AF%E5%8F%98%E6%80%A7
 * 但这会导致SimpleReservoir中的三个可变字段都要包装，不太划算
 */
impl<T> SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    pub fn get_sample_result(&self) -> SampleResult<T> {
        self.sampler
            .lock() // lock
            .unwrap()
            .get_sample_result()
    }

    pub fn try_sample(&self, element: &T) -> bool {
        self.sampler
            .lock() // lock
            .unwrap()
            .try_sample(element)
    }
}
