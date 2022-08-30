use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    data::DataCategory
};

pub trait Sensor {
    type ElementType;
    type DataType;

    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn new(name: &str, data_category: DataCategory) -> Self;
    
    fn insert(&mut self, item: &Self::DataType) -> Rc<RefCell<Self::ElementType>>;
    
    fn search(&self, item: &Self::DataType) -> Option<Rc<RefCell<Self::ElementType>>>;

    fn activate(
        &mut self, 
        item: &Self::DataType, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<Vec<Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate(
        &mut self, item: &Self::DataType, propagate_horizontal: bool, propagate_vertical: bool
    );

    fn deactivate_sensor(&mut self);
}