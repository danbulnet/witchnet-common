use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory
};

pub trait Sensor {
    type ElementType;
    type DataType;

    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert(&mut self, item: &Self::DataType) -> Rc<RefCell<Self::ElementType>>;
    
    fn search(&self, item: &Self::DataType) -> Option<Rc<RefCell<Self::ElementType>>>;

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