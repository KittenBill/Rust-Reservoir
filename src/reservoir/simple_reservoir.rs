
use rand::{
    prelude::*,
    rngs::{ ThreadRng},
};

pub struct SimpleReservoir<T> {
    sample_count: usize, // number of samples
    total: usize,        // number of elements in total

    samples: Vec<T>,
    rng: ThreadRng,
}

impl<T> SimpleReservoir<T>
where
    T: Clone,
{
    pub fn new(sample_count: usize) -> Self {
        Self {
            sample_count,
            total: 0,
            samples: Vec::with_capacity(sample_count),
            rng: thread_rng(),
        }
    }

    pub fn try_sample(&mut self, element: &T) -> bool {
        if self.total < self.sample_count {
            self.samples.push(element.clone());
            self.total += 1;
            return true;
        }

        self.total += 1;

        let coin: f64 = self.rng.gen_range(0.0..=1.0);
        if (self.sample_count as f64) / (self.total as f64) > coin {
            // keep the new element

            let idx: usize = self.rng.gen_range(0..self.sample_count);
            self.samples[idx] = element.clone();

            return true;
        }

        false
    }

    pub fn get_sample_result(&self) -> SampleResult<T> {
        // 这里要求 T 拥有 Clone 特征，就不必担心 T 没有 clone() 可以调用
        SampleResult::new(self.samples.clone(), self.total)
    }
}

#[derive(Debug)]
pub struct SampleResult<T> {
    samples: Vec<T>,
    total: usize,
}

impl<T> SampleResult<T> {
    pub fn new(samples: Vec<T>, total: usize) -> Self {
        Self { samples, total }
    }
}
