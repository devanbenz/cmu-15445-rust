macro_rules! impl_from_value_data_type_id {
    ($t:ty, $var:ident, $type_id:expr) => {
        pub(crate) impl From<$t> for ValueData {
            fn from(value: $t) -> Self {
                ValueData::$var(value)
            }
        }
        pub(crate) impl From<$t> for TypeId {
            fn from(value: $t) -> Self {
                TypeId::$type_id
            }
        }
    };
}


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

pub(crate) struct Value {
    type_id: TypeId,
    data: ValueData,
}

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

impl_from_value_data_type_id!(i8, Int8, TinyInt);

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
