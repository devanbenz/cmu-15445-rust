use std::any::TypeId;
use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::common::hash_util::{HashT, Value};

const BITSET_CAPACITY: usize = 64; // Assuming a 64-bit capacity; adjust as needed.
const HLL_CONSTANT: f64 = 0.79402; // Constant for HLL

pub struct HyperLogLog<KeyType> {
    cardinality: u64,
    n_bits: i16,
    _marker: PhantomData<KeyType>, // To mimic the template behavior
}

impl<KeyType> HyperLogLog<KeyType>
where
    KeyType: Hash + Eq + Clone + Debug,
{
    pub fn new(n_bits: i16) -> Self {
        HyperLogLog {
            cardinality: 0,
            n_bits,
            _marker: PhantomData,
        }
    }

    pub fn compute_binary(&self, hash: u64) -> u64 {
        /** @TODO(student) Implement this function! */
        0
    }

    pub fn position_of_leftmost_one(&self, bset: u64) -> u64 {
        /** @TODO(student) Implement this function! */
        0
    }

    pub fn add_elem(&mut self, val: KeyType) {
        /** @TODO(student) Implement this function! */
    }

    pub fn compute_cardinality(&mut self) {
        /** @TODO(student) Implement this function! */
    }

    pub fn get_cardinality(&self) -> u64 {
        self.cardinality
    }

    fn calculate_hash(val: KeyType) -> HashT {
        let type_id = TypeId::from(val);
        let value_obj = Value::from();
    }
}

// Explicitly instantiate the struct for i64 and String types
// In Rust, this is not necessary unless using dynamic dispatch or trait objects.



#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module where HyperLogLog is defined.

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
