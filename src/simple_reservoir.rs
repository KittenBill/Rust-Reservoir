#[allow(dead_code)]
use rand::prelude::*;

pub struct ReservoirSampler<T>
where
    T: Clone,
{
    sample_size: usize, // number of samples
    population: usize,        // number of elements in total

    samples: Vec<T>,
    rng: StdRng,
}

pub trait Sampler<T> {
    fn try_sample(&mut self, element: &T) -> bool;

    fn get_sample_result(&self) -> Result<SampleResult<T>, String>;

    fn have_sample_result(&self) -> bool;

    fn try_sample_from(&mut self, it: Box<dyn Iterator<Item = T>>){
        for element in it{
            self.try_sample(&element);
        }
    }
}

impl<T> ReservoirSampler<T>
where
    T: Clone,
{
    /// creates a simple reservoir sampler
    /// 
    /// # Panics
    /// 
    /// this function panics when sample_size is less or equal to 0
    pub fn new(sample_size: usize) -> Self {
        if sample_size <= 0 {
            panic!("sample_size should > 0");
        }
        Self {
            sample_size,
            population: 0,
            samples: Vec::with_capacity(sample_size),
            rng: StdRng::from_entropy(),
        }
    }
}

impl<T> Sampler<T> for ReservoirSampler<T>
where
    T: Clone,
{
    fn try_sample(&mut self, element: &T) -> bool {
        if self.population < self.sample_size {
            self.samples.push(element.clone());
            self.population += 1;
            return true;
        }

        self.population += 1;

        let coin: f64 = self.rng.gen_range(0.0..=1.0);
        if (self.sample_size as f64) / (self.population as f64) > coin {
            // keep the new element

            let idx: usize = self.rng.gen_range(0..self.sample_size);
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

        Ok(SampleResult::new(self.samples.clone(), self.population))
    }

    fn have_sample_result(&self) -> bool {
        self.population >= self.sample_size
    }
}

#[derive(Debug)]
pub struct SampleResult<T> {
    pub samples: Vec<T>,
    pub population: usize,
}

impl<T> SampleResult<T>
where
    T: Clone,
{
    pub fn new(samples: Vec<T>, population: usize) -> Self {
        Self { samples, population }
    }
}
