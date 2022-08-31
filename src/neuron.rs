use std::{
    rc::Rc, 
    cell::RefCell,
    hash::Hash,
    collections::HashMap
};

use crate::connection::{ Connection, ConnectionKind };

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NeuronID {
    pub id: Rc<str>,
    pub parent_id: Rc<str>
}

pub trait Neuron {
    fn id(&self) -> NeuronID;

    fn activation(&self) -> f32;

    fn is_sensor(&self) -> bool;

    fn counter(&self) -> usize;

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>;

    fn explain(&mut self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);
}

pub trait NeuronConnect {
    type NeuronType: Neuron + NeuronConnect;

    fn connect(
        &mut self, to: Rc<RefCell<Self::NeuronType>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = Self, To = Self::NeuronType>>>, String>;
}