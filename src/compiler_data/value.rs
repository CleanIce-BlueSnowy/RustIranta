//! The module of `value`

use std::fmt::Display;

pub enum Value {
    Integer(ValueInteger),
    Float(ValueFloat),
}

pub enum ValueInteger {
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Int128(i128),
    UInt128(u128),
}

pub enum ValueFloat {
    Float32(f32),
    Float64(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::Integer(int) => {
                    match int {
                        ValueInteger::Int8(int) => format!("int8({})", int),
                        ValueInteger::UInt8(int) => format!("uint8({})", int),
                        ValueInteger::Int16(int) => format!("int16({})", int),
                        ValueInteger::UInt16(int) => format!("uint16({})", int),
                        ValueInteger::Int32(int) => format!("int32({})", int),
                        ValueInteger::UInt32(int) => format!("uint32({})", int),
                        ValueInteger::Int64(int) => format!("int64({})", int),
                        ValueInteger::UInt64(int) => format!("uint64({})", int),
                        ValueInteger::Int128(int) => format!("int128({})", int),
                        ValueInteger::UInt128(int) => format!("uint128({})", int),
                    }
                }
                Self::Float(float) => {
                    match float {
                        ValueFloat::Float32(float) => format!("float32({})", float),
                        ValueFloat::Float64(float) => format!("float64({})", float),
                    }
                }
            }
        )
    }
}
