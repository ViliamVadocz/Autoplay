# Mancala

Simulating evolution to generate an agent to play the game mancala.

The current evolution process:

- Generate random agents (neural networks) for the initial population.
- Repeat until desired generation is reached:
  - Play every agent against every other, both going first and second.
  - Rank the bots by their score differential over all the games.
  - Remove some percent from the bottom of the population.
  - Repopulate by making mutated clones of the survivors, biased towards those with a greater score (fitness).
    - (The mutation process works by randomly shifting all the weights and biases on the neural network.)

Parameters to play around with:

- Neural network shape
- Number of agents in the population
- Mutation modifier
- Percentage of population that survives each generation
- Number of generation to simulate

Possible to improve the simulation:

- Less work:
  - Vary the mutation modifier with generations.
  - Introduce a few new randomly generated agents to introduce variation.
  - Mutate the entire population, excluding the top few agents.
  - Possibility of generating a completely new value for a weight when mutating.
  - Chose specific weights to mutate instead of mutating them all at once.
  - Mutate all weights by the same scalar.
  - Take two parent agents and combine their 'genes.'
  
- More work:
  - Vary the neural network shape when mutating.
  - Use N.E.A.T. instead.
  - Create a hard-coded AI to test each bot against.

Some artificial ways to improve the program:

- Make a gui for playing the game.
- Visualise the cloning process (make a graph showing which agent is descendant of which.)
- Generate a name for each agent to give them some more personality.
- Display an updating scoreboard of the current population.
  
The actual mancala game was coded by me as well, the rules are:

- 6 pits + 1 large pit (mancala) per side.
- 4 stones in each starting pit.
- On your turn, pick a pit with stones on your side of the board.
- Take all the stones out and go around the board anti-clockwise, dropping one stone in each pit you pass over.
  - (Skip opponent's mancala.)
- Some special rules apply depending on where the last stone was dropped:
  - If it landed in your mancala, you get an extra turn.
  - If it landed in an empty pit on your side opposing a non-empty pit, you capture.
    - (capture = take all the stones in the two pits and place them in your mancala.)
- When either side of the board has no more stones, the game ends.
- All the remaining stones are placed in the mancala on the same side.
- The winner is the one who has more stones in their mancala at the end of the game.
