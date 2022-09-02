use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    any::Any
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory
};

pub trait Sensor {
    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &dyn Any) -> Rc<RefCell<dyn Neuron>>;
    
    fn search(&self, item: &dyn Any) -> Option<Rc<RefCell<dyn Neuron>>>;

    fn activate(
        &mut self, 
        item: &dyn Any, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &dyn Any, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}