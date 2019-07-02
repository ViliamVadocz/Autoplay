import numpy as np
class NeuralNetwork:

    def __init__(self, layer_sizes):
        '''Initialises the neural network.'''
        self.layer_sizes = layer_sizes

        # Neurons and connections
        weight_shapes = [(a, b) for a, b in zip(self.layer_sizes[1:], self.layer_sizes[:-1])]
        self.weights = [np.random.standard_normal(s)/(s[1]**0.5) for s in weight_shapes]
        self.biases = [np.zeros((s, 1)) for s in self.layer_sizes[1:]]

        # Score
        self.gen = 0
        self.wins = 0
        self.score = 0

    def choose(self, a):
        '''Forward pass through the network to determine choice.'''
        for w, b in zip(self.weights, self.biases):
            a = self.activation(np.matmul(w, a) + b)
        return a

    @staticmethod
    def activation(x):
        '''The activation function, logistic sigmoid.'''
        return 1/(1+np.exp(-x))

    def mutate(self, mut_mod):
        '''Creates a clone of the neural network and mutates values randomly.'''
        # Creates a clone of itself and bumps up the generation.
        clone = NeuralNetwork(self.layer_sizes)
        clone.weights = self.weights
        clone.biases = self.biases
        clone.gen = self.gen + 1

        # Generates mutations for the clone.
        weight_shapes = [(a, b) for a, b in zip(self.layer_sizes[1:], self.layer_sizes[:-1])]
        weight_mutations = [np.random.standard_normal(s)/(s[1]*mut_mod) for s in weight_shapes]
        bias_mutations = [np.random.standard_normal((s, 1))/(s*mut_mod) for s in self.layer_sizes[1:]]

        # Mutates the clone
        for i in range(len(clone.weights)):
            clone.weights[i] += weight_mutations[i]
        
        for i in range(len(clone.biases)):
            clone.biases[i] += bias_mutations[i]

        # Returns mutated clone.
        return clone