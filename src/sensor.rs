use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory
};

pub trait SensorElement {}

pub trait SensorDataType {}

pub trait Sensor {
    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &dyn SensorDataType) -> Rc<RefCell<dyn SensorElement>>;
    
    fn search(&self, item: &dyn SensorDataType) -> Option<Rc<RefCell<dyn SensorElement>>>;

    fn activate(
        &mut self, 
        item: &dyn SensorDataType, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &dyn SensorDataType, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}