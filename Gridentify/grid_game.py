from dataclasses import dataclass, field
from typing import List, Set
from copy import deepcopy
import numpy as np

@dataclass
class Move:
    final: int
    used: Set[int] = field(default_factory=set)

    def __eq__(self, other):
        return self.final == other.final and self.used == other.used

    def view(self):
        """Human-friendly view of the move."""
        board = np.zeros(25, dtype=np.byte)
        for tile in self.used:
            board[tile] = 1
        board[self.final] = 2

        out = ''
        for arr in np.split(board, 5):
            # Paint by numbers.
            out += '\n' + str(arr).replace('0', ' ').replace('1', '+').replace('2', '@')
        return out

    def copy(self):
        return deepcopy(self)

class Gridentify:

    def __init__(self, board = None):
        self.score = 0
        # Generate new board if not supplied with one.
        self.board = self.new_board(5) if board is None else board

    @staticmethod
    def new_board(x: int) -> np.ndarray:
        return np.random.randint(1, 4, (x**2,), dtype=np.uint16)

    @staticmethod
    def new_num() -> np.ndarray:
        return np.random.randint(1, 4, dtype=np.uint16)

    def get_neighbours_of(self) -> List[int]:
        """Generates a list of neighbours with same value for each tile."""
        neighbours_of = []
        for i, value in enumerate(self.board):
            neighbours = []
            x = i % 5
            y = i // 5
            if x < 4 and self.board[i+1] == value: neighbours.append(i+1)
            if y < 4 and self.board[i+5] == value: neighbours.append(i+5)
            if x > 0 and self.board[i-1] == value: neighbours.append(i-1)
            if y > 0 and self.board[i-5] == value: neighbours.append(i-5)
            neighbours_of.append(neighbours)
        return neighbours_of

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
                    branch.used.add(neighbour)
                    # Add branch to moves if it is not there already.
                    if branch not in moves: moves.append(branch)
                    # Now look at this branch's neighbours...
                    discover_for(branch, neighbour)
        
        # Look at moves that end on each tile of the board.
        for i, tile in enumerate(self.board):
            discover_for(Move(i, set([i])), i)

        return moves

    def make_move(self, move: Move):
        """Makes a move in the game."""
        # Sets the final value.
        self.board[move.final] *= len(move.used)
        # Resets all used tiles except final.
        for tile in move.used: 
            if tile != move.final: self.board[tile] = self.new_num()
        # Add to score.
        self.score += self.board[move.final]

    def show_board(self):
        """Show a human-friendly view of the board."""
        print('\n')
        for arr in np.split(self.board, 5):
            print(arr)


if __name__ == "__main__":
    game = Gridentify()
    game.show_board()
    moves = game.valid_moves()
    print(f'num of valid moves: {len(moves)}')
    # Look at first five valid moves to check if they make sense.
    for move in moves[:5]:
        print(move.view())