use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory
};

pub enum SensorData {
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128), ISize(isize),
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128), USize(usize),
    F32(f32), F64(f64),
    String(String), BoxStr(Box<str>), RcStr(Rc<str>)
}

pub trait Sensor where {
    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &SensorData) -> Rc<RefCell<dyn Neuron>>;
    
    fn search(&self, item: &SensorData) -> Option<Rc<RefCell<dyn Neuron>>>;

    fn activate(
        &mut self, 
        item: &SensorData, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &SensorData, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}