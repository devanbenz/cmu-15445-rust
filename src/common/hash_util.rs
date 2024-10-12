use std::mem;
use crate::common::types::{TypeId, Value};

pub type HashT = usize;

pub const PRIME_FACTOR: HashT = 10000019;

fn hash_bytes(bytes: &[u8]) -> HashT {
    let mut hash = bytes.len();
    for &byte in bytes {
        hash = ((hash << 5) ^ (hash >> 27)) ^ (byte as HashT);
    }
    hash
}

fn combine_hashes(l: HashT, r: HashT) -> HashT {
    let both = [l, r];
    let bytes = unsafe {
        std::slice::from_raw_parts(
            both.as_ptr() as *const u8,
            mem::size_of::<HashT>() * 2,
        )
    };
    hash_bytes(bytes)
}

fn sum_hashes(l: HashT, r: HashT) -> HashT {
    (l % PRIME_FACTOR + r % PRIME_FACTOR) % PRIME_FACTOR
}

fn hash<T: Sized>(value: &T) -> HashT {
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (value as *const T) as *const u8,
            mem::size_of::<T>(),
        )
    };
    hash_bytes(bytes)
}

fn hash_ptr<T>(ptr: *const T) -> HashT {
    let address = ptr as usize;
    hash(&address)
}



pub(crate) fn hash_value(val: &Value) -> HashT {
    match val.get_type_id() {
        TypeId::TinyInt => {
            let raw = val.get_i8() as i64;
            hash(&raw)
        }
        TypeId::SmallInt => {
            let raw = val.get_i16() as i64;
            hash(&raw)
        }
        TypeId::Integer => {
            let raw = val.get_i32() as i64;
            hash(&raw)
        }
        TypeId::BigInt => {
            let raw = val.get_i64();
            hash(&raw)
        }
        TypeId::Boolean => {
            let raw = val.get_bool();
            hash(&raw)
        }
        TypeId::Decimal => {
            let raw = val.get_f64();
            hash(&raw)
        }
        TypeId::Varchar => {
            let raw = val.get_data();
            hash_bytes(raw)
        }
        TypeId::Timestamp => {
            let raw = val.get_u64();
            hash(&raw)
        }
    }
}