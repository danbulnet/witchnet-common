use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::{ Display, Formatter, Result as FmtResult } 
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

pub trait SensorData: Clone + Display + PartialOrd + PartialEq + Distance {}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum SensorDataType {
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128), ISize(isize),
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128), USize(usize),
    F32(f32), F64(f64)
}

impl Display for SensorDataType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self)
    }
}

impl Distance for SensorDataType {
    fn distance(&self, v: &Self) -> f64  {
        if *self == *v { 0.0 } else { 1.0 }
    }
}

impl<T> SensorData for T where T: Clone + Display + PartialOrd + PartialEq + Distance {}

pub trait Sensor where  {
    type Data: SensorData;

    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &Self::Data) -> Rc<RefCell<dyn Neuron>>;
    
    fn search(&self, item: &Self::Data) -> Option<Rc<RefCell<dyn Neuron>>>;

    fn activate(
        &mut self, 
        item: &Self::Data, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &Self::Data, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}