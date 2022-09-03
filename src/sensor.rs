use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    cmp::Ordering,
    any::Any
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

pub trait SensorData: Display {
    fn any(&self) -> &dyn Any;
    fn distance(&self, v: &dyn SensorData) -> f64;
    fn equals(&self, rhs: &dyn SensorData) -> bool;
    fn partial_cmp(&self, rhs: &dyn SensorData) -> Option<Ordering>;
}

impl<T: Display + PartialOrd + PartialEq + 'static> SensorData for T {
    fn any(&self) -> &dyn Any { self }

    fn distance(&self, rhs: &dyn SensorData) -> f64 {
        if *self == *rhs.any().downcast_ref::<T>().unwrap() { 0.0 } else { 1.0 }
    }

    fn equals(&self, rhs: &dyn SensorData) -> bool {
        rhs.any().downcast_ref::<T>().map(|rhs| rhs == self).unwrap_or(false)
    }
    
    fn partial_cmp(&self, rhs: &dyn SensorData) -> Option<Ordering> {
        self.partial_cmp(rhs.any().downcast_ref::<T>().unwrap())
    }
}

impl Eq for dyn SensorData {}

impl PartialEq for dyn SensorData + '_ { 
    fn eq(&self, rhs: &Self) -> bool { self.equals(rhs) }
 }

impl PartialOrd for dyn SensorData + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        self.partial_cmp(other) 
    }
}

pub trait SensorDataMarker: Clone + Display + PartialOrd + PartialEq + Distance {}

impl<T> SensorDataMarker for T 
where T: Clone + Display + PartialOrd + PartialEq + Distance {}

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