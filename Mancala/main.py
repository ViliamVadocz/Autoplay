import mancala as mncl
import neuralnetwork as nn
import numpy as np 

# Layer sizes for the network, 14 input neurons and 6 output neurons are mandatory for mancala.
layer_sizes = (14, 10, 10, 6)

# Creates the mancala game object.
game = mncl.Mancala()

# Creates initial agents.
agent0 = nn.NeuralNetwork(layer_sizes)
agent1 = nn.NeuralNetwork(layer_sizes)



# DEBUG

# A sample game with two randomly generated (dumb) agents.
while game.is_active:
    board_input = np.array(game.board).reshape((len(game.board), 1))

    if game.player == 0:
        output = agent0.choose(board_input)
        choices = np.argsort(output.reshape((1,6)))
        n = 0
        while not game.valid_choice(choices[0][n]):
            n += 1
        game.take_turn(choices[0][n])

    elif game.player == 1:
        output = agent1.choose(board_input)
        choices = np.argsort(output.reshape((1,6)))
        n = 0
        while not game.valid_choice(choices[0][n]):
            n += 1
        game.take_turn(choices[0][n])
    


