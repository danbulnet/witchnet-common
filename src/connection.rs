use std::{
    rc::Rc,
    cell::RefCell
};

use crate::neuron::{ Neuron, NeuronID };

#[derive(Debug, Clone)]
pub struct ConnectionID {
    pub from: NeuronID,
    pub to: NeuronID
}

#[derive(Copy, Clone, Debug)]
pub enum ConnectionKind {
    Definition,
    Explanation,
    Inhibition,
    Similarity,
    Consequence,
    Dummy
}

pub trait Connection {
    type From: Neuron;
    type To: Neuron;

    fn id(&self) -> ConnectionID;

    fn from(&self) -> Rc<RefCell<Self::From>>;
    
    fn to(&self) -> Rc<RefCell<Self::To>>;

    fn kind(&self) -> ConnectionKind;
    
    fn weight(&self) -> f32;
}