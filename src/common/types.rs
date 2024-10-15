/// We do a little bit of macros~~
/// This is effectively building a From trait for every single Value.
/// Needs to be used to convert bespoke rust types to our custom types.
macro_rules! impl_from_value {
    ($t:ty, $var:ident, $type_id:expr) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Self {
                    data: ValueData::$var(value),
                    type_id: $type_id
                }
            }
        }
    };
}

impl_from_value!(i8, Int8, TypeId::TinyInt);
impl_from_value!(i16, Int16, TypeId::SmallInt);
impl_from_value!(i32, Int32, TypeId::Integer);
impl_from_value!(i64, Int64, TypeId::BigInt);
impl_from_value!(bool, Bool, TypeId::Boolean);
impl_from_value!(f64, Double, TypeId::Decimal);
impl_from_value!(String, Varchar, TypeId::Varchar);
impl_from_value!(u64, UInt64, TypeId::BigInt);

/// TypeId is a generic type identifier for our DBMS.
/// Rust offers its own internal representation of types
/// so this will be used a way to map to DBMS types.
#[derive(Debug)]
pub(crate) enum TypeId {
    TinyInt,
    SmallInt,
    Integer,
    BigInt,
    Boolean,
    Decimal,
    Varchar,
    Timestamp,
}

/// ValueData will be the internal representation of a Rust primitive type.
#[derive(Debug)]
pub(crate) enum ValueData {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Bool(bool),
    Double(f64),
    Varchar(String),
    UInt64(u64),
}

/// Value is a wrapper for our actual data and mapped to a TypeId as indicated
/// in [TypeId], this is the 'date wrapper' for this.
#[derive(Debug)]
pub struct Value {
    type_id: TypeId,
    data: ValueData,
}


impl Value {
    pub(crate) fn new(type_id: TypeId, data: ValueData) -> Self {
        Value { type_id, data }
    }

    pub(crate) fn get_type_id(&self) -> &TypeId {
        &self.type_id
    }

    pub(crate) fn get_i8(&self) -> i8 {
        match &self.data {
            ValueData::Int8(v) => *v,
            _ => panic!("Invalid type for Int8"),
        }
    }

    pub(crate) fn get_i16(&self) -> i16 {
        match &self.data {
            ValueData::Int16(v) => *v,
            _ => panic!("Invalid type for Int16"),
        }
    }

    pub(crate) fn get_i32(&self) -> i32 {
        match &self.data {
            ValueData::Int32(v) => *v,
            _ => panic!("Invalid type for Int32"),
        }
    }

    pub(crate) fn get_i64(&self) -> i64 {
        match &self.data {
            ValueData::Int64(v) => *v,
            _ => panic!("Invalid type for Int64"),
        }
    }

    pub(crate) fn get_bool(&self) -> bool {
        match &self.data {
            ValueData::Bool(v) => *v,
            _ => panic!("Invalid type for Bool"),
        }
    }

    pub(crate) fn get_f64(&self) -> f64 {
        match &self.data {
            ValueData::Double(v) => *v,
            _ => panic!("Invalid type for Double"),
        }
    }

    pub(crate) fn get_u64(&self) -> u64 {
        match &self.data {
            ValueData::UInt64(v) => *v,
            _ => panic!("Invalid type for UInt64"),
        }
    }

    pub(crate) fn get_data(&self) -> &[u8] {
        match &self.data {
            ValueData::Varchar(s) => s.as_bytes(),
            _ => panic!("Invalid type for Varchar"),
        }
    }

    pub(crate) fn get_length(&self) -> usize {
        match &self.data {
            ValueData::Varchar(s) => s.len(),
            _ => panic!("Invalid type for Varchar"),
        }
    }
}
