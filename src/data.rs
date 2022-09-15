use std::{
    rc::Rc,
    marker::PhantomData,
    any::Any
};

use enum_as_inner::EnumAsInner;

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

#[derive(EnumAsInner)]
pub enum DataTypeValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    F32(f32),
    F64(f64),
    RcStr(Rc<str>),
    String(String),
    Unknown(Rc<dyn Any>)
}

impl From<bool> for DataTypeValue { 
    fn from(v: bool) -> DataTypeValue { DataTypeValue::Bool(v) } 
}

impl From<u8> for DataTypeValue { 
    fn from(v: u8) -> DataTypeValue { DataTypeValue::U8(v) } 
}

impl From<u16> for DataTypeValue { 
    fn from(v: u16) -> DataTypeValue { DataTypeValue::U16(v) } 
}

impl From<u32> for DataTypeValue { 
    fn from(v: u32) -> DataTypeValue { DataTypeValue::U32(v) } 
}

impl From<u64> for DataTypeValue { 
    fn from(v: u64) -> DataTypeValue { DataTypeValue::U64(v) } 
}

impl From<u128> for DataTypeValue { 
    fn from(v: u128) -> DataTypeValue { DataTypeValue::U128(v) } 
}

impl From<usize> for DataTypeValue { 
    fn from(v: usize) -> DataTypeValue { DataTypeValue::USize(v) } 
}

impl From<i8> for DataTypeValue { 
    fn from(v: i8) -> DataTypeValue { DataTypeValue::I8(v) } 
}

impl From<i16> for DataTypeValue { 
    fn from(v: i16) -> DataTypeValue { DataTypeValue::I16(v) } 
}

impl From<i32> for DataTypeValue { 
    fn from(v: i32) -> DataTypeValue { DataTypeValue::I32(v) } 
}

impl From<i64> for DataTypeValue { 
    fn from(v: i64) -> DataTypeValue { DataTypeValue::I64(v) } 
}

impl From<i128> for DataTypeValue { 
    fn from(v: i128) -> DataTypeValue { DataTypeValue::I128(v) } 
}

impl From<isize> for DataTypeValue { 
    fn from(v: isize) -> DataTypeValue { DataTypeValue::ISize(v) } 
}

impl From<f32> for DataTypeValue { 
    fn from(v: f32) -> DataTypeValue { DataTypeValue::F32(v) } 
}

impl From<f64> for DataTypeValue { 
    fn from(v: f64) -> DataTypeValue { DataTypeValue::F64(v) } 
}

impl From<Rc<str>> for DataTypeValue { 
    fn from(v: Rc<str>) -> DataTypeValue { DataTypeValue::RcStr(v) } 
}

impl From<String> for DataTypeValue { 
    fn from(v: String) -> DataTypeValue { DataTypeValue::String(v) } 
}

impl From<Rc<dyn Any>> for DataTypeValue { 
    fn from(v: Rc<dyn Any>) -> DataTypeValue { DataTypeValue::Unknown(v) } 
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