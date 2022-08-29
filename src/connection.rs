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
    Defining,
    Explanatory,
    Inhibitory,
    Similarity,
    Sequential
}

pub trait Connection {
    type From: Neuron + ?Sized;
    type To: Neuron + ?Sized;

    fn id(&self) -> ConnectionID;

    fn from(&self) -> Rc<RefCell<Self::From>>;
    
    fn to(&self) -> Rc<RefCell<Self::To>>;

    fn kind(&self) -> ConnectionKind;
    
    fn weight(&self) -> f32;
}