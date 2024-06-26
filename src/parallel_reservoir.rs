use std::cell::RefCell;
#[allow(dead_code)]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use rand::{prelude::*, rngs::StdRng};

use super::simple_reservoir::*;

type SamplerHandle<T> = Arc<Mutex<ReservoirSampler<T>>>;

pub struct ParallelReservoirSampler<T: Clone + Sync + Send> {
    sample_size: usize,
    samplers: Vec<SamplerHandle<T>>,
}

impl<T> ParallelReservoirSampler<T>
where
    T: Clone + Sync + Send + 'static, // todo: fix the lifetime bounds here
{
    pub fn new(sample_size: usize) -> Self {
        if sample_size <= 0 {
            panic!("sample_size should > 0");
        }
        Self {
            sample_size,
            samplers: Vec::new(),
        }
    }

    pub fn get_sampler_handle(&mut self) -> SamplerHandle<T> {
        let handle: SamplerHandle<T> = Arc::new(Mutex::new(
            ReservoirSampler::new(self.sample_size),
        ));
        self.samplers.push(handle);
        self.samplers.last().unwrap().clone()
    }

    pub fn have_sample_result(&self) -> bool {
        for handle in self.samplers.iter() {
            if !handle.lock().unwrap().have_sample_result() {
                return false;
            }
        }
        true
    }

    pub fn get_sample_result(&self) -> Result<SampleResult<T>, String> {
        if !self.have_sample_result() {
            return Err("Not enough elements in one or more Sampling Thread(s)".to_string());
        }

        let (tx, rx) = mpsc::channel();
        // this channel is utilized to simulate BlockingQueue<SampleResult<T>>

        for handle in self.samplers.iter() {
            tx.send(handle.lock().unwrap().get_sample_result()?)
                .unwrap();
        }

        let thread_count = self.samplers.len();

        let mut merger_handles = Vec::new();

        for _ in 0..thread_count - 1 {
            // 从通道中取出两个取样结果
            let result1 = rx.recv().unwrap();
            let result2 = rx.recv().unwrap();

            // 为合并线程创建发送者，以便将合并结果放入通道
            let tx_for_merger = tx.clone();

            // 创建合并线程
            let handle = thread::spawn(move || {
                tx_for_merger
                    .send(Self::merge(result1, result2))
                    .unwrap();
            }); // end of merging thread

            merger_handles.push(handle);
        }

        for handle in merger_handles {
            handle.join().unwrap();
        }

        //assert(rx中只有一个SampleResult<T>)
        Ok(rx.recv().unwrap())
    }

    fn merge(mut a: SampleResult<T>, mut b: SampleResult<T>) -> SampleResult<T> {
        let sample_size = a.samples.len();
        let mut rng = StdRng::from_entropy();
        let probability = (a.population as f64) / (a.population + b.population) as f64;

        let mut merged_samples: Vec<T> = Vec::with_capacity(sample_size);

        // closure 应用
        let mut take_randomly = |v: &mut Vec<T>, rng: &mut StdRng| {
            let idx = rng.gen_range(0_usize..v.len());
            let item = v.swap_remove(idx);
            merged_samples.push(item);
        };

        for _ in 0..sample_size {
            if rng.gen_range(0_f64..=1_f64) < probability {
                take_randomly(&mut a.samples, &mut rng);
            } else {
                take_randomly(&mut b.samples, &mut rng);
            }
        }

        SampleResult { samples: merged_samples, population: a.population + b.population }
    }
}

/*
pub struct SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    // object safe: fn new() should not be in trait Sampler
    sampler: Mutex<Box<dyn Sampler<T> + Send>>,
}
*/

/*
实现Send的类型可以在线程间安全的传递其所有权
实现Sync的类型可以在线程间安全的共享(通过引用)

考虑到SamplerHandle所做的就是包装了一个Mutex，Mutex由Sync和Send特征
所以SamplerHandle也拥有Sync和Send特征，但sampler由SimpleReservoir<T>修改为Box<dyn Sampler<T>>后，SapmlerHandle不再默认实现Sync和Send了，
原因如下：

> Any type composed entirely of Send types is automatically marked as Send as well.
> Almost all primitive types are Send, aside from raw pointers

> Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

> Implementing Send and Sync Manually Is Unsafe
> Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.

[rust book](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits)

解决方案：将定义
    sampler: Mutex<Box<dyn Sampler<T>>>,
修改为
    sampler: Mutex<Box<dyn Sampler<T> + Send>>,

 */

// unsafe impl<T> Sync for SamplerHandle<T> where T: Clone + Sync + Send {}
// unsafe impl<T> Send for SamplerHandle<T> where T: Clone + Sync + Send {}

/*
impl<T> SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    pub fn new(sampler: Box<dyn Sampler<T> + Send>) -> Self {
        Self {
            sampler: Mutex::new(sampler),
        }
    }
}
*/

/*
 * 本想给SamplerHandle实现Sampler特征，
 * 但Sampler特征中的try_sample()需要的是self的可变借用，但在Arc不允许包裹的内容被可变借用
 *
 * 一种解决方案是将Sampler中的方法改为不可变借用，再在SimpleReservoir的实现中使用RefCell以得到内部可变性
 * https://course.rs/advance/smart-pointer/cell-refcell.html#%E5%86%85%E9%83%A8%E5%8F%AF%E5%8F%98%E6%80%A7
 * 但这会导致SimpleReservoir中的三个可变字段都要包装，不太划算
 */

/*
impl<T> SamplerHandle<T>
where
    T: Clone + Sync + Send,
{
    pub fn get_sample_result(&self) -> Result<SampleResult<T>, String> {
        self.sampler
            .lock() // lock
            .unwrap()
            .get_sample_result()
    }

    pub fn have_sample_result(&self) -> bool {
        self.sampler
            .lock() //lock
            .unwrap()
            .have_sample_result()
    }

    pub fn try_sample(&mut self, element: &T) -> bool {
        self.sampler
            .lock() // lock
            .unwrap()
            .try_sample(element)
    }

    pub fn try_sample_from(&self, it: Box<dyn Iterator<Item = T>>) -> () {
        self.sampler
            .lock() // lock
            .unwrap()
            .try_sample_from(it)
    }
}
*/
