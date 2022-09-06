use std::fs::File;

use polars::prelude::*;

use crate::data::DataVec;

pub fn csv_to_dataframe(filename: &str) -> Result<DataFrame> {
    let file = File::open(filename)?;
    CsvReader::new(file).infer_schema(None).has_header(true).finish()
}

pub fn series_to_vec(series: &Series) -> DataVec {
    match series.dtype() {
        DataType::Boolean => DataVec::BoolVec(series.bool().unwrap().into_iter().collect()),
        DataType::UInt8 => DataVec::UInt8Vec(series.u8().unwrap().into_iter().collect()),
        DataType::UInt16 => DataVec::UInt16Vec(series.u16().unwrap().into_iter().collect()),
        DataType::UInt32 => DataVec::UInt32Vec(series.u32().unwrap().into_iter().collect()),
        DataType::UInt64 => DataVec::UInt64Vec(series.u64().unwrap().into_iter().collect()),
        DataType::Int8 => DataVec::Int8Vec(series.i8().unwrap().into_iter().collect()),
        DataType::Int16 => DataVec::Int16Vec(series.i16().unwrap().into_iter().collect()),
        DataType::Int32 => DataVec::Int32Vec(series.i32().unwrap().into_iter().collect()),
        DataType::Int64 => DataVec::Int64Vec(series.i64().unwrap().into_iter().collect()),
        DataType::Float32 => DataVec::Float32Vec(series.f32().unwrap().into_iter().collect()),
        DataType::Float64 => DataVec::Float64Vec(series.f64().unwrap().into_iter().collect()),
        DataType::Utf8 => DataVec::Utf8Vec(series.utf8().unwrap().into_iter().map(
            |v| match v { Some(val) => Some(val.to_string()), None => None }
        ).collect()),
        _ => DataVec::Unknown
    }
}