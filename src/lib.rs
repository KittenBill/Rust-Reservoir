mod reservoir;

#[cfg(test)]
mod tests {
    use crate::reservoir::simple_reservoir::SimpleReservoir;

    #[test]
    fn simple_reservoir_quick_test() {
        let mut sr = SimpleReservoir::new(10);
        for i in 0..100000{
            sr.try_sample(&i);
        }

        println!("{:?}", sr.get_sample_result());
    }
}