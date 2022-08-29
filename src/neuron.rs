use std::{
    rc::Rc, 
    cell::RefCell
};

use crate::connection::ConnectionKind;

#[derive(Debug, Clone)]
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
    ) -> Vec<Rc<RefCell<dyn Neuron>>>;

    fn explain(&mut self) -> Vec<Rc<RefCell<dyn Neuron>>>;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);

    fn connect(&mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind);
}