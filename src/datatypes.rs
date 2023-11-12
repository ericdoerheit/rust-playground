use std::fmt::Display;

pub enum DataType {
    // Null(),
    Boolean(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float16(f32),
    Float32(f32),
    Float64(f64),
    Utf8(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean(_) => write!(f, "{}", String::from("Hello")),

            DataType::Int8(value) => write!(f, "{}", value),
            DataType::Int16(value) => write!(f, "{}", value),
            DataType::Int32(value) => write!(f, "{}", value),
            DataType::Int64(value) => write!(f, "{}", value),
            DataType::UInt8(value) => write!(f, "{}", value),
            DataType::UInt16(value) => write!(f, "{}", value),
            DataType::UInt32(value) => write!(f, "{}", value),
            DataType::UInt64(value) => write!(f, "{}", value),
            DataType::Float16(value) => write!(f, "{}", value),
            DataType::Float32(value) => write!(f, "{}", value),
            DataType::Float64(value) =>write!(f, "{}", value),
            DataType::Utf8(value) => write!(f, "{}", value),
        }
        // write!(f, "{}", String::from("Hello"))
    }
}
