use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Display 
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

pub trait SensorData: Clone + Display + PartialOrd + PartialEq + Distance {}

impl<T> SensorData for T where T: Clone + Display + PartialOrd + PartialEq + Distance {}

pub trait SensorDataMarker {}

macro_rules! impl_sensor_data_marker {
    ( $($t:ty),* ) => { $( impl SensorDataMarker for $t {}) * }
}

impl_sensor_data_marker! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

pub trait Sensor {
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