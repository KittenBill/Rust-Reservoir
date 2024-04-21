pub mod parallel_reservoir;
pub mod simple_reservoir;
use crate::simple_reservoir::{Sampler, ReservoirSampler};
fn main() {
    let mut reservoir: ReservoirSampler<i32> = ReservoirSampler::new(10);
    for i in 0_i32..100000{
        reservoir.try_sample(&i);
    }


    println!("{:?}", reservoir.get_sample_result());

    for i in 0..1000{
        reservoir.try_sample(&i);
    }

    println!("{:?}", reservoir.get_sample_result());
}
