use std::{
    rc::Rc,
    marker::PhantomData
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataCategory {
    Numerical,
    Categorical,
    Ordinal,
}

macro_rules! impl_numerical {
    ( $($t:ty),* ) => {
        $( impl From<&$t> for DataCategory {
            fn from(_data: &$t) -> DataCategory { DataCategory::Numerical }
        }
        impl From<&[$t]> for DataCategory {
            fn from(_data: &[$t]) -> DataCategory { DataCategory::Numerical }
        }
        impl From<&[Option<$t>]> for DataCategory {
            fn from(_data: &[Option<$t>]) -> DataCategory { DataCategory::Numerical }
        }) *
    }
}

macro_rules! impl_categorical {
    ( $($t:ty),* ) => {
        $( impl From<&$t> for DataCategory {
            fn from(_data: &$t) -> DataCategory { DataCategory::Categorical }
        }
        impl From<&[$t]> for DataCategory {
            fn from(_data: &[$t]) -> DataCategory { DataCategory::Categorical }
        }
        impl From<&[Option<$t>]> for DataCategory {
            fn from(_data: &[Option<$t>]) -> DataCategory { DataCategory::Categorical }
        }) *
    }
}

impl_numerical! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl_categorical! { String, Rc<str>, bool }

pub enum DataVec {
    BoolVec(Vec<bool>),
    UInt8Vec(Vec<u8>),
    UInt16Vec(Vec<u16>),
    UInt32Vec(Vec<u32>),
    UInt64Vec(Vec<u64>),
    Int8Vec(Vec<i8>),
    Int16Vec(Vec<i16>),
    Int32Vec(Vec<i32>),
    Int64Vec(Vec<i64>),
    Float32Vec(Vec<f32>),
    Float64Vec(Vec<f64>),
    Utf8Vec(Vec<String>),
    Unknown
}

pub enum DataType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    RcStr,
    String
}

impl From<PhantomData<bool>> for DataType {
    fn from(_data_type: PhantomData<bool>) -> DataType { DataType::Bool }
}

impl From<PhantomData<u8>> for DataType {
    fn from(_data_type: PhantomData<u8>) -> DataType { DataType::U8 }
}

impl From<PhantomData<u16>> for DataType {
    fn from(_data_type: PhantomData<u16>) -> DataType { DataType::U16 }
}

impl From<PhantomData<u32>> for DataType {
    fn from(_data_type: PhantomData<u32>) -> DataType { DataType::U32 }
}

impl From<PhantomData<u64>> for DataType {
    fn from(_data_type: PhantomData<u64>) -> DataType { DataType::U64 }
}

impl From<PhantomData<u128>> for DataType {
    fn from(_data_type: PhantomData<u128>) -> DataType { DataType::U128 }
}

impl From<PhantomData<i8>> for DataType {
    fn from(_data_type: PhantomData<i8>) -> DataType { DataType::I8 }
}

impl From<PhantomData<i16>> for DataType {
    fn from(_data_type: PhantomData<i16>) -> DataType { DataType::I16 }
}

impl From<PhantomData<i32>> for DataType {
    fn from(_data_type: PhantomData<i32>) -> DataType { DataType::I32 }
}

impl From<PhantomData<i64>> for DataType {
    fn from(_data_type: PhantomData<i64>) -> DataType { DataType::I64 }
}

impl From<PhantomData<i128>> for DataType {
    fn from(_data_type: PhantomData<i128>) -> DataType { DataType::I128 }
}

impl From<PhantomData<f32>> for DataType {
    fn from(_data_type: PhantomData<f32>) -> DataType { DataType::F32 }
}

impl From<PhantomData<f64>> for DataType {
    fn from(_data_type: PhantomData<f64>) -> DataType { DataType::F64 }
}

impl From<PhantomData<Rc<str>>> for DataType {
    fn from(_data_type: PhantomData<Rc<str>>) -> DataType { DataType::RcStr }
}

impl From<PhantomData<String>> for DataType {
    fn from(_data_type: PhantomData<String>) -> DataType { DataType::String }
}