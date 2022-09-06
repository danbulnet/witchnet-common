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

impl_categorical! { String, bool }

pub enum DataVec {
    BoolVec(Vec<Option<bool>>),
    UInt8Vec(Vec<Option<u8>>),
    UInt16Vec(Vec<Option<u16>>),
    UInt32Vec(Vec<Option<u32>>),
    UInt64Vec(Vec<Option<u64>>),
    Int8Vec(Vec<Option<i8>>),
    Int16Vec(Vec<Option<i16>>),
    Int32Vec(Vec<Option<i32>>),
    Int64Vec(Vec<Option<i64>>),
    Float32Vec(Vec<Option<f32>>),
    Float64Vec(Vec<Option<f64>>),
    Utf8Vec(Vec<Option<String>>),
    Unknown
}