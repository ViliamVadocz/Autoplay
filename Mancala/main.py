import mancala as mncl
import neuralnetwork as nn
import numpy as np

# Layer sizes for the networks: 14 input neurons and 6 output neurons are mandatory for mancala, rest can be modified. 
# Hidden layer number and sizes were chosen based on intuition.
layer_sizes = (14, 10, 10, 6)

# Creates initial population of agents.
population_size = 100
population = [nn.NeuralNetwork(layer_sizes) for agent in range(100)]

# Matches up all the agents
matchups = [(a,b) for a in population for b in population]

for players in matchups:
    # Creates the mancala game object for each matchup.
    game = mncl.Mancala()

    # Agents take turns chosing stones.
    while game.is_active:
        # Takes the game board and changes into a column vector for the agents to process.
        board_input = np.array(game.board).reshape((len(game.board), 1))

        # Calculates output of the neural network for the given boardstate input.
        output = players[game.player].choose(board_input)

        # Reshapes the output into a list of prioritised choices.
        choices = np.argsort(output.reshape((1, 6)))

        # Test each choice in turn until it finds a valid move.
        n = 0
        while not game.valid_choice(choices[0][n]):
            n += 1

        # Agent takes the first valid move from prioritised choice list.
        game.take_turn(choices[0][n])

    # Adds the difference in score to agents cumulative score.
    players[0].score += game.result[0] - game.result[1]
    players[1].score += game.result[1] - game.result[0]

    # Adds a win to the appropriate agent.
    if game.result[0] > game.result[1]:
        players[0].wins += 1
    elif game.result[0] < game.result[1]:
        players[1].wins += 1

# Prints scores of each agent after first matchups
for agent,i in zip(population,range(population_size)):
    print("Agent number {} with score {} and wins {}".format(i, agent.score, agent.wins))

# DEBUG
'''
# A sample game with two randomly generated (dumb) agents.
agent0 = nn.NeuralNetwork(layer_sizes)
agent1 = nn.NeuralNetwork(layer_sizes)

while game.is_active:
    board_input = np.array(game.board).reshape((len(game.board), 1))

    if game.player == 0:
        output = agent0.choose(board_input)
        choices = np.argsort(output.reshape((1, 6)))
        n = 0
        while not game.valid_choice(choices[0][n]):
            n += 1
        game.take_turn(choices[0][n])

    elif game.player == 1:
        output = agent1.choose(board_input)
        choices = np.argsort(output.reshape((1, 6)))
        n = 0
        while not game.valid_choice(choices[0][n]):
            n += 1
        game.take_turn(choices[0][n])
'''