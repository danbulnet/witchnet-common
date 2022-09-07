use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    cmp::Ordering,
    any::Any,
    mem
};

use num_traits::ToPrimitive;

use dyn_clone::DynClone;

use crate::{
    neuron::{ Neuron, NeuronID },
    data::DataCategory
};

pub trait SensorDataBase {
    fn any(&self) -> &dyn Any;
}

impl<T> SensorDataBase for T 
where T: SensorData + Display + PartialOrd + PartialEq + Any + 'static {
    fn any(&self) -> &dyn Any { self }
}

pub trait SensorData: SensorDataBase + Display + DynClone + 'static {
    fn equals(&self, rhs: &dyn SensorData) -> bool;
    fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering>;
    fn distance(&self, v: &dyn SensorData) -> f64;
}

dyn_clone::clone_trait_object!(SensorData);

macro_rules! impl_distance_numeric {
    ( $($t:ty),* ) => {
        $( impl SensorData for $t {
            fn equals(&self, rhs: &dyn SensorData) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorData) -> f64 {
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
        $( impl SensorData for $t {
            fn equals(&self, rhs: &dyn SensorData) -> bool {
                rhs.any().downcast_ref::<$t>().map(|rhs| rhs == self).unwrap_or(false)
            }
            
            fn partial_compare(&self, rhs: &dyn SensorData) -> Option<Ordering> {
                self.partial_cmp(rhs.any().downcast_ref::<$t>().unwrap())
            }

            fn distance(&self, rhs: &dyn SensorData) -> f64 {
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
    String, Rc<str>, bool
}

impl Eq for dyn SensorData {}

impl PartialEq for dyn SensorData + '_ { 
    fn eq(&self, rhs: &Self) -> bool { self.equals(rhs) }
 }

impl PartialOrd for dyn SensorData + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        self.partial_compare(other) 
    }
}

pub trait SensorBuilder<Key: SensorData> {
    fn new(name: &str, data_category: DataCategory) -> Rc<RefCell<dyn Sensor<Data = Key>>>;
}

pub trait Sensor: Any {
    type Data: SensorData;

    fn id(&self) -> &str;

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

pub trait SensorDowncast<T: SensorData> {
    fn sensor_dynamic_downcast(
        sensor: Rc<RefCell<dyn Sensor<Data = dyn SensorData>>>
    ) -> Rc<RefCell<dyn Sensor<Data = T>>>;
}

impl<T: SensorData> SensorDowncast<T> for dyn Sensor<Data = T> {
    fn sensor_dynamic_downcast(
        sensor: Rc<RefCell<dyn Sensor<Data = dyn SensorData>>>
    ) -> Rc<RefCell<dyn Sensor<Data = T>>> {
        unsafe { 
            mem::transmute::<
                Rc<RefCell<dyn Sensor<Data = dyn SensorData>>>,
                Rc<RefCell<dyn Sensor<Data = T>>>, 
            >(sensor) 
        }
    }
}

pub trait SensorStaticDowncast<T: Sensor>  {
    fn sensor_static_downcast(
        sensor: Rc<RefCell<dyn Sensor<Data = dyn SensorData>>>
    ) -> *mut T;
}

impl<T: Sensor, D: SensorData> 
SensorStaticDowncast<T> for dyn Sensor<Data = D> {
    fn sensor_static_downcast(
        sensor: Rc<RefCell<dyn Sensor<Data = dyn SensorData>>>
    ) -> *mut T { &*sensor.borrow() as *const _ as *mut T }
}