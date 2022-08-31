use std::{
    rc::Rc, 
    cell::RefCell,
    hash::{ Hash, Hasher },
    collections::HashSet
};

use crate::connection::{ Connection, ConnectionKind };

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
    ) -> HashSet<Rc<RefCell<dyn Neuron>>>;

    fn explain(&mut self) -> HashSet<Rc<RefCell<dyn Neuron>>>;

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool);
}

pub trait NeuronConnect {
    fn connect(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>, String>;
}

struct RcRefCellNeuronHash(Rc<RefCell<dyn Neuron>>);

impl PartialEq for RcRefCellNeuronHash {
    fn eq(&self, other: &RcRefCellNeuronHash) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl Eq for RcRefCellNeuronHash {}

impl Hash for RcRefCellNeuronHash {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher { 
        hasher.write_usize(Rc::as_ptr(&self.0) as *const () as usize);
    }
}