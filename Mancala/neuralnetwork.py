import numpy as np
class NeuralNetwork:

    def __init__(self, layer_sizes, gen, other=None):
        '''Initialises the neural network.'''
        if other is not None:
            # In case of being initialised as clone.
            self.clone(other)

        else:
            # Weights and biases.
            weight_shapes = [(a, b) for a, b in zip(layer_sizes[1:], layer_sizes[:-1])]
            self.weights = ([np.random.standard_normal(s)/(s[1]**0.5) for s in weight_shapes])
            self.biases = ([np.random.standard_normal((s, 1))/(s**0.5) for s in layer_sizes[1:]])

        # Statistics.
        self.gen = gen
        self.wins = 0
        self.score = 0
        
    def clone(self, other):
        '''Copy weights and biases from other (original) to self (clone).'''
        self._weights = other.weights.copy()
        self._biases = other.biases.copy()

    @property
    def weights(self):
        '''Returns the weights of the neural network.'''
        return self._weights

    @weights.setter
    def weights(self, value):
        '''Sets the weights of the neural network'''
        self._weights = value

    @property
    def biases(self):
        '''Returns the biases of the neural network.'''
        return self._biases
    
    @biases.setter
    def biases(self, value):
        '''Sets the weights of the neural network.'''
        self._biases = value

    def choose(self, a):
        '''Forward pass through the network to determine choice.'''
        for w, b in zip(self._weights, self._biases):
            a = self.activation(np.matmul(w, a) + b)
        return a

    @staticmethod
    def activation(x):
        '''The activation function, logistic sigmoid.'''
        return 1/(1+np.exp(-x))

    def split(self, layer_sizes, mutation_mod):
        '''Creates a clone of the neural network and mutates values randomly.'''
        # Creates a clone of itself and bumps up the generation.
        clone = NeuralNetwork(layer_sizes, self.gen+1, self)

        # Generates mutations for the clone.
        weight_shapes = [(a, b) for a, b in zip(layer_sizes[1:], layer_sizes[:-1])]
        weight_mutations = [mutation_mod*np.random.standard_normal(s)/(s[1]**0.5) for s in weight_shapes]
        bias_mutations = [mutation_mod*np.random.standard_normal((s, 1))/(s**0.5) for s in layer_sizes[1:]]

        # Mutates the clone
        for i in range(len(clone._weights)):
            clone.weights[i] += weight_mutations[i]

        for i in range(len(clone.biases)):
            clone.biases[i] += bias_mutations[i]

        # Returns mutated clone.
        return clone