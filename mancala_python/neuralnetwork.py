'''The Neural Network for the game Mancala.'''

import numpy as np


class NeuralNetwork:

    def __init__(self, layer_sizes: tuple, gen: int, other=None):
        """Initialises the neural network.

        Arguments:
            layer_sizes {tuple} -- Sizes of the layers for the neural network.
            gen {int} -- The generation of the network.

        Keyword Arguments:
            other {NeuralNetwork} -- The neural network to be clones. (default: {None})
        """
        if other is not None:
            # In case of being initialised as clone.
            self.clone(other)

        else:
            # Weights and biases.
            weight_shapes = [(a, b) for a, b in zip(
                layer_sizes[1:], layer_sizes[:-1])]
            self.weights = ([np.random.standard_normal(s)/(s[1]**0.5)
                             for s in weight_shapes])
            self.biases = ([np.random.standard_normal((s, 1))/(s**0.5)
                            for s in layer_sizes[1:]])

        # Statistics.
        self.gen = gen
        self.wins = 0
        self.score = 0

    def clone(self, other):
        """Copy weights and biases from other (original) to self (clone).

        Arguments:
            other {NeuralNetwork} -- The neural network being cloned.
        """
        self._weights = other.weights.copy()
        self._biases = other.biases.copy()

    @property
    def weights(self):
        """returns the weights of the neural network.

        Returns:
            list of np.arrays -- The weights of the neural network.
        """
        return self._weights

    @weights.setter
    def weights(self, value):
        """Sets the weights of the neural network

        Arguments:
            value {list of np.arrays} -- The value to set the weights to.
        """

        self._weights = value

    @property
    def biases(self):
        """Returns the biases of the neural network.

        Returns:
            list of np.arrays -- The biases of the neural network.
        """
        return self._biases

    @biases.setter
    def biases(self, value):
        """Sets the weights of the neural network.

        Arguments:
            value {list of np.arrays} -- The value to set the biases to.
        """
        self._biases = value

    def choose(self, a):
        """Forward pass through the network to determine choice.

        Arguments:
            a {np.array} -- The input array for the neural network. Expects shape to be (14, 1).

        Returns:
            [np.array] -- The output array of the neural network. Shape should be (6, 1).
        """
        for w, b in zip(self._weights, self._biases):
            a = self.activation(np.matmul(w, a) + b)
        return a

    @staticmethod
    def activation(x: float):
        """The activation function, logistic sigmoid.

        Arguments:
            x {float} -- A number.

        Returns:
            float -- The logistic sigmoid of x.
        """
        return 1/(1+np.exp(-x))

    def split(self, layer_sizes: tuple, mutation_mod: float):
        """Creates a clone of the neural network and mutates values randomly.

        Arguments:
            layer_sizes {tuple} -- Sizes of the layers for the neural network.
            mutation_mod {float} -- The mutation modifier. Multiplies the weight and bias mutations.

        Returns:
            NeuralNetwork -- The mutated clone of the neural network calling this function.
        """
        # Creates a clone of itself and bumps up the generation.
        clone = NeuralNetwork(layer_sizes, self.gen+1, self)

        # Generates mutations for the clone.
        weight_shapes = [(a, b)
                         for a, b in zip(layer_sizes[1:], layer_sizes[:-1])]
        weight_mutations = [
            mutation_mod*np.random.standard_normal(s)/(s[1]**0.5) for s in weight_shapes]
        bias_mutations = [
            mutation_mod*np.random.standard_normal((s, 1))/(s**0.5) for s in layer_sizes[1:]]

        weights = clone.weights
        biases = clone.biases

        # Mutates the clone
        for i in range(len(weights)):
            weights[i] = np.add(weights[i], weight_mutations[i])

        for i in range(len(biases)):
            biases[i] = np.add(biases[i], bias_mutations[i])

        clone.weights = weights
        clone.biases = biases
        # Returns mutated clone.
        return clone
