use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

pub trait Connection {
    fn from<T>(&self) -> Rc<RefCell<T>> where T: Neuron;
    fn to<T>(&self) -> Rc<RefCell<T>> where T: Neuron;
}