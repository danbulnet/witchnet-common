use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    neuron::Neuron,
    connection::{ Connection, ConnectionKind, ConnectionID }
};

pub struct InhibitoryConnection<From: Neuron, To: Neuron> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>,
    weight: f32
}

impl<From: Neuron, To: Neuron> InhibitoryConnection<From, To> {
    pub fn new(
        from: Rc<RefCell<From>>, to: Rc<RefCell<To>>, weight: f32
    ) -> InhibitoryConnection<From, To> {
        InhibitoryConnection { from, to, weight }
    }
}

impl<From: Neuron, To: Neuron> Connection for InhibitoryConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { ConnectionKind::Inhibitory }
    
    fn weight(&self) -> f32 { self.weight }
}