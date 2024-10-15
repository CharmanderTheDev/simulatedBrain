use std::collections::HashMap;

struct Synapse {
    input: Neuron,
    output: Neuron,
    id: usize,

    weight: f32,

    transmitters: f32,
} impl Synapse {

    fn tick(mut self) {
        self.output.add_potential(self.transmitters);
        self.transmitters = 0.0;
        if self.weight<1.0/(self.input.get_synapse_amount()*100.0){self.input.get_brain().remove_synapse(self.id);}
    }

    fn set_weight(mut self, amount: f32) {
        self.input.add_weight_total(amount - self.weight);
        self.weight = amount;
    }

    fn transmit(mut self, amount: f32) {
        self.transmitters += amount;
    }

    fn adjustWeight(mut self, dopamine: f32, heat: f32){
        self.set_weight(self.weight/(
                              (1.0 as f32)/
        (self.input.get_synapse_amount()*self.weight).pow(dopamine * heat)));
    }
}

struct Neuron {

    brain: Brain,
    id: usize,

    heat: f32,
    potential: f32,
    synapses: HashMap<usize, Synapse,>,

    synapse_amount: usize,
    weight_total: f32,
}
impl Neuron {

    fn normalize(self) {
        for id in self.synapses {
            
        }
    }
}

struct Brain {}
impl Brain {}

fn main() {
    println!("Hello, world!");
}
