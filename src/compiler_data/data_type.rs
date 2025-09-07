//! The module of data types

use std::collections::HashMap;
use std::fmt::Display;
use maplit::hashmap;

pub struct TypeInterner {
    type_cnt: u32,
    pub to_data_type: HashMap<TypeId, DataType>,
}

impl TypeInterner {
    #[must_use]
    pub fn create() -> Self {
        Self {
            type_cnt: TypeId::BUILTIN_MAX,
            to_data_type: hashmap! {
                TypeId::VOID => DataType {
                    name: "void".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::INT8 => DataType {
                    name: "int8".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::UINT8 => DataType {
                    name: "uint8".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::INT16 => DataType {
                    name: "int16".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::UINT16 => DataType {
                    name: "uint16".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::INT32 => DataType {
                    name: "int32".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::UINT32 => DataType {
                    name: "uint32".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::INT64 => DataType {
                    name: "int64".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::UINT64 => DataType {
                    name: "uint64".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::INT128 => DataType {
                    name: "int128".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::FLOAT32 => DataType {
                    name: "float32".to_string(),
                    desc: TypeDesc::Raw,
                },
                TypeId::FLOAT64 => DataType {
                    name: "float64".to_string(),
                    desc: TypeDesc::Raw,
                },
            },
        }
    }

    #[must_use]
    pub fn new_type(&mut self, data_type: DataType) -> TypeId {
        self.type_cnt += 1;
        let new_id = TypeId(self.type_cnt);
        self.to_data_type.insert(new_id, data_type);
        new_id
    }
}

pub struct DataType {
    name: String,
    desc: TypeDesc,
}

pub enum TypeDesc {
    Raw,
    Struct,
    Enum,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct TypeId(u32);

impl TypeId {
    pub const VOID: Self = Self(0);
    pub const INT8: Self = Self(1);
    pub const UINT8: Self = Self(2);
    pub const INT16: Self = Self(3);
    pub const UINT16: Self = Self(4);
    pub const INT32: Self = Self(5);
    pub const UINT32: Self = Self(6);
    pub const INT64: Self = Self(7);
    pub const UINT64: Self = Self(8);
    pub const INT128: Self = Self(9);
    pub const UINT128: Self = Self(10);
    pub const FLOAT32: Self = Self(11);
    pub const FLOAT64: Self = Self(12);

    const BUILTIN_MAX: u32 = 12;
}
