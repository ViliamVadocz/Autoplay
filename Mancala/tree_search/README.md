# Mancala

My second attempt at solving Mancala with machine learning.

Uses tree search to explore possible moves up to a set depth, then uses a neural network to evaluate the position.
This is then used to determine what move to chose.

TODO:

- Remake Mancala using numpy (to increase speed)
- Implement tree search using alpha-beta pruning
- Use a basic board state evaluation to start (captured stones - enemy stones)
- Train a neural network to approximate it
- Amplification, Distillation?

https://www.youtube.com/watch?v=l-hh51ncgDI
