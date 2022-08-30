use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

pub trait Sensor {
    type ElementType;
    type DataType;

    fn name(&self) -> &str;
    
    fn new(name: &str) -> Self;
    
    fn insert(&mut self, item: &Self::DataType) -> Rc<RefCell<Self::ElementType>>;
    
    fn search(&self, item: &Self::DataType) -> Option<Rc<RefCell<Self::ElementType>>>;

    fn activate(
        &mut self, 
        item: &Self::DataType, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Vec<Rc<RefCell<dyn Neuron>>>;

    fn deactivate(&mut self, propagate_vertical: bool);
}