struct NeuronID {
    id: String,
    parent_id: String
}

pub trait Neuron {
    fn get_id(&self) -> NeuronID;

    fn get_activation(&self) -> f32;

    fn stimulate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> f32;
}