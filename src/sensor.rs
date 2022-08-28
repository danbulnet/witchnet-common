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

    fn stimulate(
        &mut self, 
        item: &Self::DataType, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Rc<RefCell<Self::ElementType>>;
}