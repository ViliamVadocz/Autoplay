'''Main file. Run this if you want to evolve some agents. You can also tweak the parameters below for different results.'''

import mancala as mncl
import neuralnetwork as nn
import poparchives as popa
import numpy as np

# ---------------------------------------------------------------------------

# PARAMETERS:

# Layer sizes for the networks: 14 input neurons and 6 output neurons are mandatory for mancala, rest can be modified.
# Hidden layer number and sizes were chosen based on intuition.
layer_sizes: tuple = (14, 10, 10, 6)

# Population size determines number of agents in each generation.
# If you are changing the population size, make sure to delete the archived populations or they will load instead and everything will break.
# TODO Fix above mentioned problem
population_size: int = 300

assert population_size >= 20, "Population size is too small."

# Mutation modifier (Larger number means larger mutations).
mutation_mod: float = 0.5

# Percentage of populaiton to keep in each generation.
# Has to be more than zero otherwise the population will never evolve.
# Has to be less than one otherwise all agents are removed and there is no way to repopulate.
thanos_mod: float = 0.8

assert 0 < thanos_mod < 1.0, "Thanos modifier is either too large or too small."

# Start and end points for the evolution.
# Set start_gen to largest reached generation before break to automatically load from where you left off.
start_gen: int = 0
end_gen: int = 176

# ---------------------------------------------------------------------------

# POPULATION LOADING / CREATION:

# Loading population data from file or creating a new population.
filename: str = "gen" + str(start_gen) + ".pkl"
try:
    popa.load(filename)

except:
    # Creates a new population if it cannot load from file.
    print("Creating a brand new population. Let there be light!")
    population = [nn.NeuralNetwork(layer_sizes, 0)
                  for agent in range(population_size)]
    popa.write(population, filename)

else:
    # Loads population from an existing file.
    print("Loading existing population from file: {}".format(filename))
    population = popa.load(filename)

# ---------------------------------------------------------------------------

# EVOLUTION LOOP:

for gen in range(start_gen, end_gen):

    # Resets agent score so old agents don't get an advantage.
    for agent in population:
        agent.score = 0

    # Matches up all the agents.
    matchups = [(a, b) for a in population for b in population]

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

    # Doing like Thanos and snapping away half* of the population.
    # *This now depends on the thanos modifier.
    survivors = int(population_size * thanos_mod)
    population = population[:survivors]

    # Creating a list of probabilities of being cloned for each survivor based on their fitness.
    # Accounts for negative score by subracting the last agent's score.
    score_sum = sum(
        (agent.score - population[-1].score) for agent in population)
    assert score_sum != 0, "The population has degenerated and the score sum is 0."
    clone_probs = [(agent.score - population[-1].score) /
                   score_sum for agent in population]

    # Creating clones from survivors.
    clones = [np.random.choice(population, p=clone_probs).split(
        layer_sizes, mutation_mod) for clone in range(population_size-len(population))]
    ### clones = [agent.split(mutation_mod) for agent in population]

    # Adding clones to population.
    population.extend(clones)

    # Progress report
    print("Generation {} done!".format(gen + 1))
    top_agent = population[0]
    print("The top agent was a {} generation agent with {} total wins.".format(
        top_agent.gen, top_agent.wins))

    # Records the new generation.
    filename = "gen" + str(gen+1) + ".pkl"
    popa.write(population, filename)

    # DEBUG
    '''
    test = popa.load(filename)
    
    for p1, p2 in zip(population, test):
        w1 = []
        for entry in p1.weights:
            w1 = np.concatenate((w1,np.ravel(entry)))
        w2 = []
        for entry in p2.weights:
            w2 = np.concatenate((w2, np.ravel(entry)))
        
        diff = sum(np.subtract(w1, w2))
        
        if np.size(w1) != np.size(w2):
            print("SIZE",np.size(w1),"DID NOT MATCH",np.size(w2))
        if diff != 0:
            print("DIFF NOT ZERO!",diff)
    '''


print("Done!")
