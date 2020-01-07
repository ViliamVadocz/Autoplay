from dataclasses import dataclass, field
from typing import List, Tuple
from copy import deepcopy
import numpy as np
import time

from grid_game import Gridentify
import seeded_bot

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

    def __init__(self, board = None, seed = None):
        self.score = 0
        # Generate new seed if needed.
        if seed is None:
            ii32 = np.iinfo(np.int32)
            seed = np.random.randint(1, ii32.max)
            print(f'Generated random seed: {seed}')
        else:
            print(f'Seed: {seed}')
        self.seed = seed
        # Generate new board if not supplied with one.
        self.board = self.new_board(5) if board is None else board

    def new_board(self, x: int) -> np.ndarray:
        board = np.empty((x**2,), dtype=np.uint16)
        for i in range(x**2):
            board[i] = self.new_num()
        return board

    def new_num(self) -> int:
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
    # Start a timer.
    start_time = time.time()

    # Make new game.
    test_seed =  20766236554
    # print(f'seed: {test_seed}')
    game = SeededGridentify(seed=test_seed)
    game.show_board()

    # Initial moves.
    valid_moves = game.valid_moves()

    move_num = 0
    while len(valid_moves) > 0:
        move_num += 1
        print(f'\n--- Move #{move_num} ---')
        print(f'Number of valid moves: {len(valid_moves)}')

        move = Move(-1)
        while move not in valid_moves:
            # THIS IS WHERE THE MOVE MACHINE GOES.            
            board = game.board.copy()

            num_ok_moves = seeded_bot.eval_num_moves(board, game.seed)
            if num_ok_moves > 0:
                a = int(20/num_ok_moves)
                # a = max(0, int(5 - num_ok_moves/5))
                # a = int(100/len(valid_moves))
            else:
                a = 100
            depth = min(a, 3) + 2
            print(f'Depth for next move: {depth}')
            evaluation, move = seeded_bot.tree_search(board, game.seed, depth=depth)
            print(f'Move eval: {evaluation}')
        
        # Show the game.
        print(move.view())
        game.make_move(move)
        game.show_board()
        print(f'\nScore: {game.score}')
        # Get new valid moves.
        valid_moves = game.valid_moves()
        
    print('\nGame Over')

    # End the timer
    end_time = time.time()

    seconds = end_time - start_time
    minutes = seconds // 60
    seconds %= 60

    print(f'Time: {int(minutes)}m {int(seconds)}s')