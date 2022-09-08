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

impl !UnknownDataTypeMarker for PhantomData<bool> {}
impl !UnknownDataTypeMarker for PhantomData<u8> {}
impl !UnknownDataTypeMarker for PhantomData<u16> {}
impl !UnknownDataTypeMarker for PhantomData<u32> {}
impl !UnknownDataTypeMarker for PhantomData<u64> {}
impl !UnknownDataTypeMarker for PhantomData<u128> {}
impl !UnknownDataTypeMarker for PhantomData<usize> {}
impl !UnknownDataTypeMarker for PhantomData<i8> {}
impl !UnknownDataTypeMarker for PhantomData<i16> {}
impl !UnknownDataTypeMarker for PhantomData<i32> {}
impl !UnknownDataTypeMarker for PhantomData<i64> {}
impl !UnknownDataTypeMarker for PhantomData<i128> {}
impl !UnknownDataTypeMarker for PhantomData<isize> {}
impl !UnknownDataTypeMarker for PhantomData<f32> {}
impl !UnknownDataTypeMarker for PhantomData<f64> {}
impl !UnknownDataTypeMarker for PhantomData<Rc<str>> {}
impl !UnknownDataTypeMarker for PhantomData<String> {}

pub trait DataDeductor { 
    fn data_type(&self) -> DataType;
    fn data_category(&self) -> DataCategory;
 }

impl DataDeductor for bool {
    fn data_type(&self) -> DataType { DataType::Bool }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for u8 {
    fn data_type(&self) -> DataType { DataType::U8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u16 {
    fn data_type(&self) -> DataType { DataType::U16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u32 {
    fn data_type(&self) -> DataType { DataType::U32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u64 {
    fn data_type(&self) -> DataType { DataType::U64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for usize {
    fn data_type(&self) -> DataType { DataType::U128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for u128 {
    fn data_type(&self) -> DataType { DataType::USize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i8 {
    fn data_type(&self) -> DataType { DataType::I8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i16 {
    fn data_type(&self) -> DataType { DataType::I16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i32 {
    fn data_type(&self) -> DataType { DataType::I32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i64 {
    fn data_type(&self) -> DataType { DataType::I64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for i128 {
    fn data_type(&self) -> DataType { DataType::I128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for isize {
    fn data_type(&self) -> DataType { DataType::ISize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for f32 {
    fn data_type(&self) -> DataType { DataType::F32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for f64 {
    fn data_type(&self) -> DataType { DataType::F64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for Rc<str> {
    fn data_type(&self) -> DataType { DataType::RcStr }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for String {
    fn data_type(&self) -> DataType { DataType::String }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<bool> {
    fn data_type(&self) -> DataType { DataType::Bool }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<u8> {
    fn data_type(&self) -> DataType { DataType::U8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u16> {
    fn data_type(&self) -> DataType { DataType::U16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u32> {
    fn data_type(&self) -> DataType { DataType::U32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u64> {
    fn data_type(&self) -> DataType { DataType::U64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<usize> {
    fn data_type(&self) -> DataType { DataType::U128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<u128> {
    fn data_type(&self) -> DataType { DataType::USize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i8> {
    fn data_type(&self) -> DataType { DataType::I8 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i16> {
    fn data_type(&self) -> DataType { DataType::I16 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i32> {
    fn data_type(&self) -> DataType { DataType::I32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i64> {
    fn data_type(&self) -> DataType { DataType::I64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<i128> {
    fn data_type(&self) -> DataType { DataType::I128 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<isize> {
    fn data_type(&self) -> DataType { DataType::ISize }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<f32> {
    fn data_type(&self) -> DataType { DataType::F32 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<f64> {
    fn data_type(&self) -> DataType { DataType::F64 }
    fn data_category(&self) -> DataCategory { DataCategory::Numerical }
}

impl DataDeductor for PhantomData<Rc<str>> {
    fn data_type(&self) -> DataType { DataType::RcStr }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}

impl DataDeductor for PhantomData<String> {
    fn data_type(&self) -> DataType { DataType::String }
    fn data_category(&self) -> DataCategory { DataCategory::Categorical }
}