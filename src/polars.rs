use polars_core::series::Series;

pub fn series_to_vec(series: &Series) -> SensorDataVec {
    match series.dtype() {
        DataType::Boolean => SensorDataVec::BoolVec(series.bool().unwrap().into_iter().collect()),
        DataType::UInt8 => SensorDataVec::UInt8Vec(series.u8().unwrap().into_iter().collect()),
        DataType::UInt16 => SensorDataVec::UInt16Vec(series.u16().unwrap().into_iter().collect()),
        DataType::UInt32 => SensorDataVec::UInt32Vec(series.u32().unwrap().into_iter().collect()),
        DataType::UInt64 => SensorDataVec::UInt64Vec(series.u64().unwrap().into_iter().collect()),
        DataType::Int8 => SensorDataVec::Int8Vec(series.i8().unwrap().into_iter().collect()),
        DataType::Int16 => SensorDataVec::Int16Vec(series.i16().unwrap().into_iter().collect()),
        DataType::Int32 => SensorDataVec::Int32Vec(series.i32().unwrap().into_iter().collect()),
        DataType::Int64 => SensorDataVec::Int64Vec(series.i64().unwrap().into_iter().collect()),
        DataType::Float32 => 
            SensorDataVec::Float32Vec(series.f32().unwrap().into_iter().collect()),
        DataType::Float64 => 
            SensorDataVec::Float64Vec(series.f64().unwrap().into_iter().collect()),
        DataType::Utf8 => SensorDataVec::Utf8Vec(series.utf8().unwrap().into_iter().map(
            |v| match v { Some(val) => Some(val.to_string()), None => None }
        ).collect()),
        _ => SensorDataVec::Unknown
    }
}