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
        board = np.zeros((5,5), dtype=np.byte)
        for tile in self.used:
            x = tile % 5
            y = tile // 5
            board[x, y] = 1

        x = self.final % 5
        y = self.final // 5
        board[x, y] = 2
            
        # Paint by numbers.
        out = '\n '+ str(board)[1:-1]\
            .replace('0', ' ')\
            .replace('1', '+')\
            .replace('2', '@')
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
        for i, value in enumerate(self.board):
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
        board = self.board.reshape((5,5))
        print('\n ' + str(board)[1:-1])


if __name__ == "__main__":
    test = np.array([
        3,   3,   1,   1,   3,
        3,   3,   2,   3,   1,
        1,   1,   2,   2,   1,
        1,   1,   3,   2,   1,
        3,   1,   1,   1,   2
    ])
    game = Gridentify(board=test)
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
    