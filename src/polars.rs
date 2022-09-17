use std::fs::File;

use polars::prelude::*;

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

pub fn csv_to_dataframe(filename: &str) -> Result<DataFrame> {
    let file = File::open(filename)?;
    CsvReader::new(file).infer_schema(None).has_header(true).finish()
}

pub fn series_to_datavec(series: &Series) -> Result<DataVec> {
    match series.dtype() {
        DataType::UInt8 => Ok(DataVec::UInt8Vec(
            series.u8()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt16 => Ok(DataVec::UInt16Vec(
            series.u16()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt32 => Ok(DataVec::UInt32Vec(
            series.u32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::UInt64 => Ok(DataVec::UInt64Vec(
            series.u64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int8 => Ok(DataVec::Int8Vec(
            series.i8()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int16 => Ok(DataVec::Int16Vec(
            series.i16()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int32 => Ok(DataVec::Int32Vec(
            series.i32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Int64 => Ok(DataVec::Int64Vec(
            series.i64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Float32 => Ok(DataVec::Float32Vec(
            series.f32()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Float64 => Ok(DataVec::Float64Vec(
            series.f64()?.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        )),
        DataType::Utf8 => Ok(DataVec::Utf8Vec(
            series.utf8()?.into_iter()
                .filter(|x| x.is_some()).map(|x| x.unwrap().to_string()).collect()
        )),
        _ => Ok(DataVec::Unknown)
    }
}