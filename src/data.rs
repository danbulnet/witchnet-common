use std::rc::Rc;

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
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    F32,
    F64,
    RcStr,
    String,
    Unknown
}

pub auto trait UnknownDataTypeMarker {}

impl !UnknownDataTypeMarker for bool {}
impl !UnknownDataTypeMarker for u8 {}
impl !UnknownDataTypeMarker for u16 {}
impl !UnknownDataTypeMarker for u32 {}
impl !UnknownDataTypeMarker for u64 {}
impl !UnknownDataTypeMarker for u128 {}
impl !UnknownDataTypeMarker for usize {}
impl !UnknownDataTypeMarker for i8 {}
impl !UnknownDataTypeMarker for i16 {}
impl !UnknownDataTypeMarker for i32 {}
impl !UnknownDataTypeMarker for i64 {}
impl !UnknownDataTypeMarker for i128 {}
impl !UnknownDataTypeMarker for isize {}
impl !UnknownDataTypeMarker for f32 {}
impl !UnknownDataTypeMarker for f64 {}
impl !UnknownDataTypeMarker for Rc<str> {}
impl !UnknownDataTypeMarker for String {}

pub trait DataTypeMarker { fn data_type(&self) -> DataType; }

impl<T: UnknownDataTypeMarker> DataTypeMarker for T {
    fn data_type(&self) -> DataType { DataType::Unknown }
}

impl DataTypeMarker for bool {
    fn data_type(&self) -> DataType { DataType::Bool }
}

impl DataTypeMarker for u8 {
    fn data_type(&self) -> DataType { DataType::U8 }
}

impl DataTypeMarker for u16 {
    fn data_type(&self) -> DataType { DataType::U16 }
}

impl DataTypeMarker for u32 {
    fn data_type(&self) -> DataType { DataType::U32 }
}

impl DataTypeMarker for u64 {
    fn data_type(&self) -> DataType { DataType::U64 }
}

impl DataTypeMarker for u128 {
    fn data_type(&self) -> DataType { DataType::USize }
}

impl DataTypeMarker for usize {
    fn data_type(&self) -> DataType { DataType::U128 }
}

impl DataTypeMarker for i8 {
    fn data_type(&self) -> DataType { DataType::I8 }
}

impl DataTypeMarker for i16 {
    fn data_type(&self) -> DataType { DataType::I16 }
}

impl DataTypeMarker for i32 {
    fn data_type(&self) -> DataType { DataType::I32 }
}

impl DataTypeMarker for i64 {
    fn data_type(&self) -> DataType { DataType::I64 }
}

impl DataTypeMarker for i128 {
    fn data_type(&self) -> DataType { DataType::I128 }
}

impl DataTypeMarker for isize {
    fn data_type(&self) -> DataType { DataType::ISize }
}

impl DataTypeMarker for f32 {
    fn data_type(&self) -> DataType { DataType::F32 }
}

impl DataTypeMarker for f64 {
    fn data_type(&self) -> DataType { DataType::F64 }
}

impl DataTypeMarker for Rc<str> {
    fn data_type(&self) -> DataType { DataType::RcStr }
}

impl DataTypeMarker for String {
    fn data_type(&self) -> DataType { DataType::String }
}