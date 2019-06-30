import mancala as mncl
import neuralnetwork as nn
import poparchives as popa
import numpy as np

# ---------------------------------------------------------------------------

# PARAMETERS:

# Layer sizes for the networks: 14 input neurons and 6 output neurons are mandatory for mancala, rest can be modified. 
# Hidden layer number and sizes were chosen based on intuition.
layer_sizes = (14, 10, 10, 6)

# Population size should be an even number.
population_size = 10

# Mutation modifier (Larger number means smaller mutations).
mutation_mod = 5

# Start and end points for the evolution.
# Set start_gen to largest reached generation before break to automatically load from where you left off.
start_gen = 0
end_gen = 5

# ---------------------------------------------------------------------------

# Loading population data from file or creating a new population.
file = "gen" + str(start_gen) + ".pkl"
try:
    popa.load(file)
except:
    # Creates a new population if it cannot load from file.
    population = [nn.NeuralNetwork(layer_sizes) for agent in range(population_size)]
else:
    # Loads population from an existing file.
    population = popa.load(file)

# ---------------------------------------------------------------------------

# EVOLUTION LOOP
for gen in range(start_gen, end_gen):

    # Resets agent score so old agents don't get an advantage.
    for agent in population:
        agent.score = 0

    # Matches up all the agents.
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

    # Sorting the population by score.
    population.sort(key=lambda x: x.score, reverse=True)

    # Doing like Thanos and snapping away half of the population.
    half = int(population_size / 2)
    population = population[:half]

    # Creating clones from survivors.
    clones = []
    for agent in population:
        clones.append(agent.mutate(mutation_mod))

    # Adding clones to population.
    population = [*population, *clones]

    # Records the new generation.
    file = "gen" + str(gen + 1) + ".pkl"
    popa.write(population,file)
    

print("Done!")