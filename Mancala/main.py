import mancala as mncl
import neuralnetwork as nn
import numpy as np 

# Layer sizes for the network, 14 input neurons and 6 output neurons are mandatory for mancala.
layer_sizes = (14, 10, 10, 6)

# Creates the mancala game object.
game = mncl.Game

# Creates initial agents.
agent0 = nn.NeuralNetwork(layer_sizes)
agent1 = nn.NeuralNetwork(layer_sizes)



# DEBUG

# A sample game with two randomly generated (dumb) agents.
while game.is_active:
    if game.player == 0:
        choice = agent0.choose(game.board)
    elif game.player == 1:
        choice = agent1.choose(game.board)
    game.take_turn(np.argmax(choice))
    


