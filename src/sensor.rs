use std::{
    rc::Rc,
    cell::RefCell
};

pub trait Sensor {
    type ElementType;
    type DataType;

    fn name(&self) -> &str;
    
    fn new(name: &str) -> Self;
    
    fn insert(&mut self, item: &Self::DataType) -> Rc<RefCell<Self::ElementType>>;
    
    fn search(&self, item: &Self::DataType) -> Option<Rc<RefCell<Self::ElementType>>>;
}