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
    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_to_connection(
        &mut self, to_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_from(
        &mut self, from: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;

    fn connect_from_connection(
        &mut self, from_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;
}

pub trait NeuronConnectBilateral<To: Neuron + NeuronConnect>: Neuron + NeuronConnect {
    fn connect_bilateral(&mut self, to: Rc<RefCell<To>>, kind: ConnectionKind) 
    -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = dyn Neuron>>>, String>;
}