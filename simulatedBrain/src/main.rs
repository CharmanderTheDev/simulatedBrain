use std::collections::HashMap;

struct Synapse {
    input: Neuron,
    output: Neuron,
    id: usize,

    weight: f32,

    transmitters: f32,
} impl Synapse {

    fn new(input: Neuron, output: Neuron, id: usize) -> Synapse{
        let mut synapse = Synapse{input:input,output:output,id:id,weight:0.0,transmitters:0.0}
        synapse.set_weight(1.0)
    }

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

    fn adjust_weight(mut self, dopamine: f32, heat: f32) {
        self.set_weight(self.weight/(((1.0 as f32)/(self.input.get_synapse_amount()*self.weight)).powf(dopamine * heat)));
    }

    fn get_weight(self) -> f32 {self.weight}
}

struct Neuron {

    brain: Brain,
    id: usize,

    heat: f32,
    potential: f32,
    synapses: HashMap<usize, Synapse>,

    synapse_amount: usize,
    weight_total: f32,
}
impl Neuron {

    fn new(brain: Brain, id: usize) -> Neuron{
        Neuron {
            brain:brian, 
            id:id, 
            heat:0.0, 
            potential:0.0, 
            synapses:HashMap::new(), 
            synapse_amount:0, 
            weight_total:0.0
        };
    }

    fn normalize(&mut self) {
        for (id, synapse) in self.synapses {
            synapse.setWeight(synapse.getWeight/self.weight_total);
        }

        self.weight_total = self.synapse_amount;
    }

    fn add_potential(&mut self, amount:f32) {
        self.potential += amount;
    }

    fn fire(&mut self) {
        self.normalize();
        for (_id, synapse) in self.synapses {
            synapse.transmit(synapse.get_weight());
        }
    }

    fn add_synapse(&mut self, synapse: Synapse) {
        self.synapse_amount += 1;
        self.synapses[&synapse.id] = synapse;
    }

    fn remove_synapse(&mut self, id: usize) {
        self.synapse_amount -= 1;
        self.synapses.remove(&id);
    }

    fn tick(&mut self) {

        if self.potential >= 1.0 {
            self.fire();
        }

        if self.heat <= 0.25 {
            self.brain.grow_synapse(self.id);
        }

        self.heat /= 2;
    }
}

struct Brain<'a> {
    neurons: [Box<&'a mut Neuron>],
    neuron_count: u32,
    synapses: HashMap<usize, Synapse>,
    dead_synapses: [usize],

    inputs: [usize],
    input_amount: usize,
    outputs: [usize],

    synapse_table: Hashmap<usize, HashMap<usize, Synapse>>,
    num_synapses: u32,
}
impl Brain {
    
    fn grow_synapse(&mut self, input: [i32]) {
        let output = 
    }
}

fn main() {
    println!("Hello, world!");
}
