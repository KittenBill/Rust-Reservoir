#[allow(dead_code)]
use rand::prelude::*;

pub struct SimpleReservoir<T>
where
    T: Clone,
{
    sample_count: usize, // number of samples
    total: usize,        // number of elements in total

    samples: Vec<T>,
    rng: StdRng,
}

pub trait Sampler<T> {
    fn try_sample(&mut self, element: &T) -> bool;

    fn get_sample_result(&self) -> Result<SampleResult<T>, String>;

    fn have_sample_result(&self) -> bool;

    fn try_sample_from(&mut self, mut it: Box<dyn Iterator<Item = T>>) -> (){
        while let Some(element) = it.next() {
            self.try_sample(&element);
        }
    }
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
            rng: StdRng::from_entropy(),
        }
    }
}

impl<T> Sampler<T> for SimpleReservoir<T>
where
    T: Clone,
{
    fn try_sample(&mut self, element: &T) -> bool {
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

    fn get_sample_result(&self) -> Result<SampleResult<T>, String> {
        // 这里要求 T 拥有 Clone 特征，就不必担心 T 没有 clone() 可以调用
        if !self.have_sample_result() {
            return Err("Not enough elements to sample for a SimpleReservoir".to_string());
        }

        Ok(SampleResult::new(self.samples.clone(), self.total))
    }

    fn have_sample_result(&self) -> bool {
        self.total >= self.sample_count
    }
}

#[derive(Debug)]
pub struct SampleResult<T> {
    pub samples: Vec<T>,
    pub total: usize,
}

impl<T> SampleResult<T>
where
    T: Clone,
{
    pub fn new(samples: Vec<T>, total: usize) -> Self {
        Self { samples, total }
    }
}
