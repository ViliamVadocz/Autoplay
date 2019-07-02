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
population_size: int = 1000

if population_size < 200:
    print("Population size is too small. Aborting...")
    exit()

# Mutation modifier (Larger number means larger mutations).
mutation_mod = 10

# Percentage of populaiton to keep in each generation.
# Has to be more than zero otherwise the population will never evolve.
# Has to be less than one otherwise all agents are removed and there is no way to repopulate.
thanos_mod: float = 0.5 # Currently has to be 0.5 because new cloning doesn't seem to work properly.

if not 0 < thanos_mod < 1.0:
    print("Thanos modifier is either too large or too small. Aborting...")
    exit()

# Start and end points for the evolution.
# Set start_gen to largest reached generation before break to automatically load from where you left off.
start_gen = 0
end_gen = 10

# ---------------------------------------------------------------------------

# Loading population data from file or creating a new population.
filename = "gen" + str(start_gen) + ".pkl"
try:
    popa.load(filename)

except:
    # Creates a new population if it cannot load from file.
    print("Creating a brand new population. Let there be light!")
    population = [nn.NeuralNetwork(layer_sizes) for agent in range(population_size)]
    popa.write(population, filename)

else:
    # Loads population from an existing file.
    print("Loading existing population from file: {}".format(filename))
    population = popa.load(filename)

# ---------------------------------------------------------------------------

# EVOLUTION LOOP
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
        print(game.result)

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

    # TODO Make this work:
    ### # Creating a list of probabilities of being cloned for each survivor based on their fitness.
    ### # Accounts for negative score by subracting the last agent's score.
    ### score_sum = sum((agent.score - population[-1].score) for agent in population)
    ### clone_probs = [(agent.score - population[-1].score) / score_sum for agent in population]

    # Creating clones from survivors.
    ### clones = [np.random.choice(population, p=clone_probs).mutate(mutation_mod) for clone in range(population_size-len(population))]
    clones = [agent.mutate(mutation_mod) for agent in population]

    # Adding clones to population.
    population.extend(clones)

    # Progress report
    print("Generation {} done!".format(gen))
    top_agent = population[0]
    print("Top agent was a generation {} agent with {} score and {} total wins.".format(
        top_agent.gen, top_agent.score, top_agent.wins))

    # Records the new generation.
    filename = "gen" + str(gen+1) + ".pkl"
    popa.write(population, filename)


print("Done!")
