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
        }) *
    }
}

impl_numerical! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl_categorical! { String, bool }