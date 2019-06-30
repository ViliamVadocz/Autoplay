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
