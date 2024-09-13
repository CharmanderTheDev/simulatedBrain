import numpy as np
import random
import math

#we're building a brain

class Synapse:

    def __init__(self, input, output, id):

        self.input = input
        self.output = output
        self.id = id

        self.weight = 1
        self.transmitters = 0
    
    def tick(self):
        self.output.addPotential(self.transmitters)
        self.transmitters = 0
    
    def setWeight(self, amount):
        self.input.weighttotal += (amount - self.weight)
        self.weight = amount

    def transmit(self, amount):
        self.transmitters+=amount
    
    def adjustWeight(self, dopamine, heat):
        self.setWeight(self.weight/((1/self.input.synapseAmount*self.weight)**(dopamine*heat)))

class Neuron:

    def __init__(self, brain, id):

        self.brain = brain
        self.id = id

        self.heat = 0
        self.potential = 0
        self.synapses = []
        self.synapseAmount = 0
        self.weighttotal = 0

    def normalize(self):
        for synapse in self.synapses:
            synapse.weight/=self.weighttotal
        
        self.weighttotal = len(self.synapses)

    def addPotential(self, amount):
        self.potential+=amount
        
    def fire(self):
        self.normalize()
        for synapse in self.synapses:
            synapse.transmit(synapse.weight)
        
        if self.synapseAmount>=0:self.heat+=1

    def addSynapse(self, synapse):
        self.synapseAmount+=1
        self.synapses.append(synapse)

    def tick(self):
        #firing if potential is sufficient
        if(self.potential>=1):
            self.fire()
        
        #growing new neurons during inactivity
        if(self.heat<=.25):
            self.brain.growSynapse(self.id)

        #cooling off!
        self.heat/=2



class Brain:

    def __init__(self, neuronCount, inputs, outputs):
        self.neurons = list(Neuron(self, id) for id in range(neuronCount))
        self.neuronCount = neuronCount
        self.synapses = {}

        self.inputs = inputs
        self.inputAmount = len(inputs)
        self.outputs = outputs

        #index in synapseTable indicates input
        #index in subdictionary indicates output
        self.synapseTable = {}
        self.numSynapses = 0

    def growSynapse(self, input):
        output = random.randint(0,self.neuronCount-1)

        #swap in/out randomly
        if(random.choice([True,False])):
            temp = input
            input = output
            output = temp
        
        #does the input exist in the synapse table?
        if input in self.synapseTable:
            #does the output exist in the synapse table? if not, add it!
            if not (output in self.synapseTable[input]):
                self.__addSynapse(input, output)
            
            #here it already exists, so we don't add it at all 
        else:
            #here the input dict does not exist, so we must initialize it first, then add the synapse
            self.synapseTable[input] = {}
            self.__addSynapse(input, output)

    def __addSynapse(self, input, output):
        #initializing synapse
        synapse = Synapse(self.neurons[input], self.neurons[output], len(self.synapses))

        #adding synapse to in-out lookup table
        self.synapseTable[input][output] = synapse

        #adding synapse to master list (for ticking)
        self.synapses[self.numSynapses] = synapse

        #hooking synapse to neuron
        self.neurons[input].addSynapse(synapse)

        #updating synapse count
        self.numSynapses+=1

    def removeSynapse(self, id):
        synapse = self.synapses[id]

        #removing from synapse table
        del self.synapseTable[synapse.input][synapse.output]

        #unhooking from parent neuron
        synapse.input.remove

        #removing from synapse list
        del self.synapses[id]
        

    def next(self, inputs, dopamine=0, fullReturn=False):

        #loading inputs
        for x in range(0,self.inputAmount):
            self.neurons[x].potential += inputs[x]

        #ticking neurons
        for neuron in self.neurons:neuron.tick()

        #iterating through synapses
        for neuron in self.neurons:
            for synapse in neuron.synapses:

                #ticking synapses
                synapse.tick()

                #updating synapse weights
                synapse.adjustWeight(dopamine, neuron.heat)

        #returning outputs
        return(list(lambda output: self.neurons[output] for output in self.outputs))

brain = Brain(100, [0,1,2], [99])
outputs = [0]
while True:
    outputs = brain.next([1,1,1], dopamine=outputs[0])
    print(outputs[0])