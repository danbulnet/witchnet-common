use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    cmp::Ordering,
    any::Any
};

use num_traits::ToPrimitive;

use dyn_clone::DynClone;

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory,
    distances::Distance
};

// Dynamic

pub trait SensorDataDynamicBase {
    fn any(&self) -> &dyn Any;
}

impl<T> SensorDataDynamicBase for T 
where T: SensorDataDynamic + Debug + PartialOrd + PartialEq + 'static {
    fn any(&self) -> &dyn Any { self }
}

pub trait SensorDataDynamic: SensorDataDynamicBase + Debug + DynClone + 'static {
    fn equals(&self, rhs: &dyn SensorDataDynamic) -> bool;
    fn partial_compare(&self, rhs: &dyn SensorDataDynamic) -> Option<Ordering>;
    fn distance(&self, v: &dyn SensorDataDynamic) -> f64;
}

dyn_clone::clone_trait_object!(SensorDataDynamic);

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
                    (
                        Self::to_f64(self).unwrap_unchecked() - 
                        Self::to_f64(&rhs).unwrap_unchecked()
                    ).abs()
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

impl<T: SensorDataDynamic + Debug + PartialOrd + PartialEq + 'static> SensorDataDynamic for Rc<RefCell<T>> {
    fn equals(&self, rhs: &dyn SensorDataDynamic) -> bool {
        rhs.any().downcast_ref::<T>().map(|rhs| *rhs == *self.borrow()).unwrap_or(false)
    }
    
    fn partial_compare(&self, rhs: &dyn SensorDataDynamic) -> Option<Ordering> {
        (*self.borrow()).partial_cmp(rhs.any().downcast_ref::<T>().unwrap())
    }

    fn distance(&self, rhs: &dyn SensorDataDynamic) -> f64 {
        if *self.borrow() == *rhs.any().downcast_ref::<T>().unwrap() { 0.0 } else { 1.0 }
    }
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


pub trait SensorDynamicBuilder<Key: SensorDataDynamic> {
    fn new(name: &str, data_category: DataCategory) -> Rc<RefCell<dyn SensorDynamic<Data = Key>>>;
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

// Fast

pub trait SensorDataFast: Debug + Distance + PartialEq + PartialOrd + Copy {}

impl<T> SensorDataFast for T 
where T: Debug + Distance + PartialEq + PartialOrd + Copy {}

pub trait SensorFast {
    type Data: SensorDataFast; 

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