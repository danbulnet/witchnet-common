use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    cmp::Ordering,
    any::Any
};

use num_traits::ToPrimitive;

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

pub trait SensorDataDynamicBase {
    fn any(&self) -> &dyn Any;
}

pub trait SensorDataDynamic: SensorDataDynamicBase + Display {
    fn equals(&self, rhs: &dyn SensorDataDynamic) -> bool;
    fn partial_compare(&self, rhs: &dyn SensorDataDynamic) -> Option<Ordering>;
    fn distance(&self, v: &dyn SensorDataDynamic) -> f64;
}

impl<T: Display + PartialOrd + PartialEq + 'static> SensorDataDynamicBase for T {
    fn any(&self) -> &dyn Any { self }
}

macro_rules! impl_distance_numeric {
    ( $($t:ty),* ) => {
        $( impl SensorDataDynamic for $t {
            fn equals(&self, rhs: &dyn SensorDataDynamic) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorDataDynamic) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorDataDynamic) -> f64 {
                let rhs = *rhs.any().downcast_ref::<$t>().unwrap();
                unsafe { 
                    (Self::to_f64(self).unwrap_unchecked() - Self::to_f64(&rhs).unwrap_unchecked()).abs()
                }
            }
        }) *
    }
}

macro_rules! impl_distance_categoric {
    ( $($t:ty),* ) => {
        $( impl SensorDataDynamic for $t {
            fn equals(&self, rhs: &dyn SensorDataDynamic) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorDataDynamic) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorDataDynamic) -> f64 {
                if *self == *rhs.any().downcast_ref::<$t>().unwrap() { 0.0 } else { 1.0 }
            }
        }) *
    }
}

impl_distance_numeric! { 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

impl_distance_categoric! {
    String
}

impl Eq for dyn SensorDataDynamic {}

impl PartialEq for dyn SensorDataDynamic + '_ { 
    fn eq(&self, rhs: &Self) -> bool { self.equals(rhs) }
 }

impl PartialOrd for dyn SensorDataDynamic + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        self.partial_compare(other) 
    }
}

pub trait SensorDataFastMarker: Display + Distance + PartialEq + PartialOrd + Clone {}

impl<T> SensorDataFastMarker for T 
where T: Display + Distance + PartialEq + PartialOrd + Clone {}

pub trait SensorDataDynamicMarker: SensorDataDynamic {}

impl<T> SensorDataDynamicMarker for T 
where T: SensorDataDynamic {}

pub trait SensorDynamicBuilder {
    fn new(name: &str, data_category: DataCategory) -> Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>;
}

pub trait SensorDynamic {
    type Data: SensorDataDynamic; 

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

pub trait SensorFast {
    type Data: SensorDataFastMarker; 

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