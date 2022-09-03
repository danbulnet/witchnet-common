use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Display ,
    any::Any,
    cmp::Ordering
};

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

// pub trait SensorDataMarker {}

// macro_rules! impl_sensor_data_marker {
//     ( $($t:ty),* ) => { $( impl SensorDataMarker for $t {}) * }
// }

// impl_sensor_data_marker! { 
//     i8, i16, i32, i64, i128, isize,
//     u8, u16, u32, u64, u128, usize,
//     f32, f64,
//     String, str
// }

// pub trait SensorData: Display {
//     fn any(&self) -> &dyn Any;
//     fn distance(&self, v: &dyn SensorData) -> f64;
//     fn equals(&self, rhs: &dyn SensorData) -> bool;
//     fn partial_cmp(&self, rhs: &dyn SensorData) -> Option<Ordering>;
// }

// impl<T: Display + PartialOrd + PartialEq + 'static> SensorData for T {
//     fn any(&self) -> &dyn Any { self }

//     fn distance(&self, rhs: &dyn SensorData) -> f64 {
//         if *self == *rhs.any().downcast_ref::<T>().unwrap() { 0.0 } else { 1.0 }
//     }

//     fn equals(&self, rhs: &dyn SensorData) -> bool {
//         rhs.any().downcast_ref::<T>().map(|rhs| rhs == self).unwrap_or(false)
//     }
    
//     fn partial_cmp(&self, rhs: &dyn SensorData) -> Option<Ordering> {
//         self.partial_cmp(rhs.any().downcast_ref::<T>().unwrap())
//     }
// }

// impl Eq for dyn SensorData {}

// impl PartialEq for dyn SensorData + '_ { 
//     fn eq(&self, rhs: &Self) -> bool { self.equals(rhs) }
//  }

// impl PartialOrd for dyn SensorData + '_ {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
//         self.partial_cmp(other) 
//     }
// }

// pub trait DistanceX where {
//     fn any(&self) -> &dyn Any;
//     fn distance(&self, v: &dyn DistanceX) -> f64;
//     fn equals(&self, rhs: &dyn DistanceX) -> bool;
// }

// impl<T: 'static + PartialEq> DistanceX for T {
//     // Co-opting the compiler's inference was easier than figuring out the
//     // direct incantation.
//     fn any(&self) -> &dyn Any {
//         self
//     }

//     // Note here that we're downcasting to T and not dyn FooTrait or Box<dyn FooTrait>
//     fn distance(&self, rhs: &dyn DistanceX) -> f64 {
//         if *self == *rhs.any().downcast_ref::<T>() { 0.0 } else { 1.0 }
//     }

//     fn equals(&self, rhs: &dyn DistanceX) -> bool {
//         rhs.any().downcast_ref::<T>().map(|rhs| rhs == self).unwrap_or(false)
//     }
// }

pub trait SensorData: Clone + Display + PartialOrd + PartialEq + Distance {}

impl<T> SensorData for T
where T: Clone + Display + PartialOrd + PartialEq + Distance {}

pub trait Sensor {
    fn name(&self) -> &str;

    fn data_category(&self) -> DataCategory;
    
    fn insert<Data: SensorData>(&mut self, item: &Data) -> Rc<RefCell<dyn Neuron>>;
    
    fn search<Data: SensorData>(&self, item: &Data) -> Option<Rc<RefCell<dyn Neuron>>>;

    fn activate<Data: SensorData>(
        &mut self, 
        item: &Data, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String>;
    
    fn deactivate<Data: SensorData>(
        &mut self, item: &Data, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<(), String>;

    fn deactivate_sensor(&mut self);
}