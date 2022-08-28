#[allow(dead_code)]
pub struct NeuronID {
    pub id: String,
    pub parent_id: String
}

pub trait Neuron {
    fn get_id(&self) -> NeuronID;

    fn activation(&self) -> f32;

    fn stimulate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32;
}