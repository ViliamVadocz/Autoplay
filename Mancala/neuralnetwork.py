import numpy as np
class NeuralNetwork:

    def __init__(self, layer_sizes):
        '''Initialises the neural network.'''
        # Neurons and connections
        weight_shapes = [(a, b) for a, b in zip(layer_sizes[1:], layer_sizes[:-1])]
        self.weights = [np.random.standard_normal(s)/(s[1]**0.5) for s in weight_shapes]
        self.biases = [np.zeros((s, 1)) for s in layer_sizes[1:]]

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
        # Creates exact clone of itself, resets wins and bumps up the generation.
        clone = self
        clone.wins = 0
        clone.gen += 1

        # Mutates the clone.
        weight_mutations = [np.random.standard_normal(w.shape)/(w.shape[1]*mut_mod) for w in clone.weights]
        bias_mutations = [np.random.standard_normal(b.shape)/(b.shape[1]*mut_mod) for b in clone.biases]

        for i in range(len(clone.weights)):
            clone.weights[i] += weight_mutations[i]
        
        for i in range(len(clone.biases)):
            clone.biases[i] += bias_mutations[i]

        # Returns mutated clone.
        return clone