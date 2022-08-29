use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct NeuronID {
    pub id: Rc<str>,
    pub parent_id: Rc<str>
}

pub trait Neuron {
    fn id(&self) -> NeuronID;

    fn activation(&self) -> f32;

    fn stimulate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32;

    fn is_sensor(&self) -> bool;

    fn counter(&self) -> usize;
}