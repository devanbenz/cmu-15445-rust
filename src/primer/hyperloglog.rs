use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use fixedbitset::FixedBitSet;
use crate::common::hash_util::{hash_value, HashT};
use crate::common::types::Value;

const BITSET_CAPACITY: usize = 64;

/// The HLL constant is just a correction value that was
/// found in the following paper: https://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf
const HLL_CONSTANT: f64 = 0.79402;

/// HyperLogLog is an algorithm used for tracking unique values
/// in a data stream also known as the cardinality of data. This is
/// done without storing the data in memory so it's able to track
/// the cardinality of very large data sets and use little to no
/// space to do so.
///
/// Description of HLL algorithm
/// Given the following inputs:
/// - b - Number of initial bits in a binary representation of a hash value
/// - m - number of registers (or also called buckets) - can be considered as a memory block.
/// They are equal to 2^b. (The terms "buckets" and "registers" can be used interchangeably
/// when discussing HyperLogLog and tasks).
/// - p - leftmost position of 1 (MSBs position of 1)
///
/// So lets say given b = 3, m will then equal 2^3 so m = 8. We have 8 buckets.
/// Given the following hash(val) = 1011 0010 0000 0000 0000 0000 0000 0000 1000 0000 0000
///
/// 101 1 0010 0000 0000 0000 0000 0000 0000 1000 0000 0000
/// ^   ---------------------------------------------------
/// |________________________________           |
/// We reserve the first 3 bits ----|          |
///                                           |
/// We initialize 2^3 registers              |
/// { 0, 0, 0, 0, 0, 0, 0, 0 }              |
///                                        V
/// 1 0010 0000 0000 0000 0000 0000 0000 1000 0000 0000
/// ^
/// |_______________________________________________
/// Now we need to find the LMB (Left most bit) ---/
///
/// Our left most bit is 1; we get p as the index for 101 (first b(3) bits)
/// p = 5, so we will set register[p] = 1 so register[5] = 1.
///
/// Our new register set is:
/// { 0, 0, 0, 0, 1, 0, 0, 0 }
///
/// So for this hash we have the following
///  m = 8; p = 5; b = 3
///
/// The HLL computation step is as follows with the data gathered:
/// HLL = 0.79402*8*(8 / val = for (int i = 0; i < 8; i++) { 1/2^register[i] }))
pub struct HyperLogLog<KeyType> {
    cardinality: u64,
    n_bits: i16,
    registers: Vec<u8>,
    _marker: PhantomData<KeyType>, // Need to mark T as generic types
}

impl<KeyType> HyperLogLog<KeyType>
where
    KeyType: Hash + Eq + Clone + Debug, Value: From<KeyType>
{
    pub fn new(n_bits: i16) -> Self {
        let b = (2_u32.pow(n_bits as u32)) as usize;
        let mut registers = Vec::with_capacity(b);
        registers.fill(0);
        HyperLogLog {
            cardinality: 0,
            n_bits,
            registers,
            _marker: PhantomData,
        }
    }

    /// add_elem will sum the hashes to an ongoing number. In this case
    /// we will use the cardinality field.
    pub fn add_elem(&mut self, val: KeyType) {
        let hash = Self::calculate_hash(val);
        todo!()
    }

    pub fn compute_cardinality(&mut self) {
        todo!()
    }

    pub fn get_cardinality(&self) -> u64 {
        self.cardinality
    }

    fn compute_binary(&self, hash: HashT) -> FixedBitSet {
        let b = vec![hash];
        FixedBitSet::with_capacity_and_blocks(64, b)
    }

    fn position_of_leftmost_one(&self, bset: FixedBitSet) -> u64 {
        todo!()
    }

    fn calculate_hash(val: KeyType) -> HashT {
        let value_obj = Value::from(val);
        hash_value(&value_obj)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratch() {
        let mut obj = HyperLogLog::<String>::new(1);
        assert_eq!(obj.get_cardinality(), 0);
        obj.add_elem("Welcome to CMU DB (15-445/645)".to_string());

        obj.compute_cardinality();

        let ans = obj.get_cardinality();
        assert_eq!(ans, 2);
    }

    #[test]
    fn basic_test1() {
        let mut obj = HyperLogLog::<String>::new(1);
        assert_eq!(obj.get_cardinality(), 0);
        obj.add_elem("Welcome to CMU DB (15-445/645)".to_string());

        obj.compute_cardinality();

        let ans = obj.get_cardinality();
        assert_eq!(ans, 2);

        for i in 0..10u64 {
            obj.add_elem("Andy".to_string());
            obj.add_elem("Connor".to_string());
            obj.add_elem("J-How".to_string());
            obj.add_elem("Kunle".to_string());
            obj.add_elem("Lan".to_string());
            obj.add_elem("Prashanth".to_string());
            obj.add_elem("William".to_string());
            obj.add_elem("Yash".to_string());
            obj.add_elem("Yuanxin".to_string());
            if i == 0 {
                obj.compute_cardinality();
                let ans = obj.get_cardinality();
                assert_eq!(ans, 6);
            }
        }

        obj.compute_cardinality();
        let ans = obj.get_cardinality();
        assert_eq!(ans, 6);
    }

    #[test]
    fn basic_test2() {
        let mut obj = HyperLogLog::<i64>::new(3);

        assert_eq!(obj.get_cardinality(), 0);

        obj.add_elem(0);

        obj.compute_cardinality();
        let ans = obj.get_cardinality();

        assert_eq!(ans, 7);

        for i in 0..10u64 {
            obj.add_elem(10);
            obj.add_elem(122);
            obj.add_elem(200);
            obj.add_elem(911);
            obj.add_elem(999);
            obj.add_elem(1402);
            obj.add_elem(15445);
            obj.add_elem(15645);
            obj.add_elem(123456);
            obj.add_elem(312457);
            if i == 0 {
                obj.compute_cardinality();
                let ans = obj.get_cardinality();
                assert_eq!(ans, 10);
            }
        }

        for i in 0..10u64 {
            obj.add_elem(-1);
            obj.add_elem(-2);
            obj.add_elem(-3);
            obj.add_elem(-4);
            obj.add_elem(-5);
            obj.add_elem(-6);
            obj.add_elem(-7);
            obj.add_elem(-8);
            obj.add_elem(-9);
            obj.add_elem(-27);
            if i == 0 {
                obj.compute_cardinality();
                let ans = obj.get_cardinality();
                assert_eq!(ans, 10);
            }
        }
        obj.compute_cardinality();
        let ans = obj.get_cardinality();
        assert_eq!(ans, 10);
    }

    #[test]
    fn edge_test1() {
        let mut obj = HyperLogLog::<i64>::new(-2);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 0);
    }

    #[test]
    fn edge_test2() {
        let mut obj = HyperLogLog::<i64>::new(0);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 0);

        obj.add_elem(1);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 1665180);

        obj.add_elem(-1);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 1665180);
    }

    #[test]
    fn basic_parallel_test() {
        let obj = Arc::new(Mutex::new(HyperLogLog::<String>::new(1)));

        let mut threads1 = vec![];
        for _ in 0..10 {
            let obj_clone = Arc::clone(&obj);
            threads1.push(thread::spawn(move || {
                let mut obj = obj_clone.lock().unwrap();
                obj.add_elem("Welcome to CMU DB (15-445/645)".to_string());
            }));
        }

        for thread in threads1 {
            thread.join().unwrap();
        }

        {
            let mut obj = obj.lock().unwrap();
            obj.compute_cardinality();
            let ans = obj.get_cardinality();
            assert_eq!(ans, 2);
        }

        let mut threads2 = vec![];
        for _ in 0..10 {
            let names = vec![
                "Andy", "Connor", "J-How", "Kunle", "Lan",
                "Prashanth", "William", "Yash", "Yuanxin",
            ];
            for name in &names {
                let obj_clone = Arc::clone(&obj);
                let name = name.to_string();
                threads2.push(thread::spawn(move || {
                    let mut obj = obj_clone.lock().unwrap();
                    obj.add_elem(name);
                }));
            }
        }

        for thread in threads2 {
            thread.join().unwrap();
        }

        {
            let mut obj = obj.lock().unwrap();
            obj.compute_cardinality();
            let ans = obj.get_cardinality();
            assert_eq!(ans, 6);
        }
    }

    #[test]
    fn parallel_test1() {
        let obj = Arc::new(Mutex::new(HyperLogLog::<String>::new(14)));

        let mut threads1 = vec![];
        for _ in 0..10 {
            let obj_clone = Arc::clone(&obj);
            threads1.push(thread::spawn(move || {
                let mut obj = obj_clone.lock().unwrap();
                obj.add_elem("Welcome to CMU DB (15-445/645)".to_string());
            }));
        }

        for thread in threads1 {
            thread.join().unwrap();
        }

        {
            let mut obj = obj.lock().unwrap();
            obj.compute_cardinality();
            let ans = obj.get_cardinality();
            assert_eq!(ans, 13009);
        }

        let mut threads2 = vec![];
        for _ in 0..3000 {
            let names = vec!["Andy", "Connor", "J-How", "Kunle"];
            for name in &names {
                let obj_clone = Arc::clone(&obj);
                let name = name.to_string();
                threads2.push(thread::spawn(move || {
                    let mut obj = obj_clone.lock().unwrap();
                    obj.add_elem(name);
                }));
            }
        }

        for thread in threads2 {
            thread.join().unwrap();
        }

        {
            let mut obj = obj.lock().unwrap();
            obj.compute_cardinality();
            let ans = obj.get_cardinality();
            assert_eq!(ans, 13010);
        }
    }

    // Additional tests for HyperLogLogPresto can be added similarly.
}
