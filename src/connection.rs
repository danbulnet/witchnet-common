use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::Neuron;

pub enum ConnectionType {
    Definition,
    Explanation,
    Inhibition,
    Similarity,
    Consequence
}

pub trait Connection {
    type From: Neuron;
    type To: Neuron;

    fn from(&self) -> Rc<RefCell<Self::From>>;
    
    fn to(&self) -> Rc<RefCell<Self::To>>;

    fn weight(&self) -> f32;

    fn connection_type(&self) -> ConnectionType;
}