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

pub trait Sensor where  {
    type DataType: Clone + Display + PartialOrd + PartialEq + Distance;

    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &Self::DataType) -> Rc<RefCell<dyn Neuron>>;
    
    fn search(&self, item: &Self::DataType) -> Option<Rc<RefCell<dyn Neuron>>>;

    fn activate(
        &mut self, 
        item: &Self::DataType, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &Self::DataType, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}

pub trait SensorMarker: Sensor {}