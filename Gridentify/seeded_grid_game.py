from dataclasses import dataclass, field
from typing import List
from copy import deepcopy
import numpy as np

from grid_game import Gridentify

@dataclass
class Move:
    final: int
    used: List[int] = field(default_factory=list)

    def __eq__(self, other):
        return self.final == other.final and self.used == other.used

    def view(self):
        """Human-friendly view of the move."""
        board = np.zeros((5,5), dtype=np.byte)
        for order, tile in enumerate(self.used):
            x = tile % 5
            y = tile // 5
            board[y, x] = len(self.used) - order
            
        # Get rid of zeroes.
        out = '\n '+ str(board)[1:-1].replace('0', ' ')
        return out

    def copy(self):
        return deepcopy(self)

class SeededGridentify(Gridentify):

    def __init__(self, seed = None, board = None):
        self.score = 0
        # Generate new seed if needed.
        if seed is None:
            ii32 = np.iinfo(np.int32)
            seed = np.random.randint(1, ii32.max)
            print(f'Generated random seed: {seed}')
        self.seed = seed
        # Generate new board if not supplied with one.
        self.board = self.new_board(5) if board is None else board

    def new_board(self, x: int) -> np.ndarray:
        board = np.empty((x**2,), dtype=np.uint16)
        for i in range(x**2):
            board[i] = self.new_num()
        return board

    def new_num(self) -> np.ndarray:
        """Really bad randomness, same as in the original game."""
        e = (16807 * self.seed) % 1924421567
        self.seed = e if e > 0 else e + 3229763266
        num = (e % 3) + 1
        return num

    def valid_moves(self) -> List[Move]:
        """Generates all valid moves on current board."""
        neighbours_of = self.get_neighbours_of()
        moves = []

        def discover_for(move: Move, tile: int):
            """Recursive function to search for all moves."""
            # Look at good neighbours of current tile.
            for neighbour in neighbours_of[tile]:
                # If the neighbour is not part of the move yet, 
                # make a new branch.
                if neighbour not in move.used:
                    branch = move.copy()
                    branch.used.append(neighbour)
                    # Add branch to moves if it is not there already.
                    if branch not in moves: moves.append(branch)
                    # Now look at this branch's neighbours...
                    discover_for(branch, neighbour)
        
        # Look at moves that end on each tile of the board.
        for i, value in enumerate(self.board):
            discover_for(Move(i, [i]), i)

        return moves

    def make_move(self, move: Move):
        """Makes a move in the game."""
        # Sets the final value.
        self.board[move.final] *= len(move.used)
        # Resets all used tiles except final (in select-order).
        for tile in move.used[::-1]:
            if tile != move.final: self.board[tile] = self.new_num()
        # Add to score.
        self.score += self.board[move.final]


if __name__ == "__main__":
    test = np.array([
        3,   3,   1,   1,   3,
        3,   3,   2,   3,   1,
        1,   1,   2,   2,   1,
        1,   1,   3,   2,   1,
        3,   1,   1,   1,   2
    ])
    game = SeededGridentify(board=test, seed=123)
    valid_moves = game.valid_moves()

    # print(len(valid_moves))

    move = Move(-1)

    while len(valid_moves) > 0:
        game.show_board()

        while move not in valid_moves:
            # THIS IS WHERE THE MOVE MACHINE SHOULD GO.
            move = valid_moves[0] # Always picking first move.

        print(move.view())
        game.make_move(move)
        valid_moves = game.valid_moves()
        
    print(f'Score: {game.score}')
    