mod reservoir;
use crate::reservoir::simple_reservoir::SimpleReservoir;

fn main() {
    let mut reservoir = SimpleReservoir::new(10);
    for i in 0..100000{
        reservoir.try_sample(&i);
    }

    println!("{:?}", reservoir.get_sample_result());

    for i in 0..100000{
        reservoir.try_sample(&i);
    }

    println!("{:?}", reservoir.get_sample_result());
}
