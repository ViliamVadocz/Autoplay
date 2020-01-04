from dataclasses import dataclass, field
from typing import List, Set
import numpy as np

@dataclass
class Move:
    final: int
    used: Set[int] = field(default_factory=set)

    def __eq__(self, other):
        return self.final == other.final and self.used == other.used

    def view(self):
        board = np.zeros(25, dtype=np.byte)
        for tile in self.used:
            board[tile] = 1
        board[self.final] = 2

        out = ''
        for arr in np.split(board, 5):
            out += '\n' + str(arr).replace('0', ' ').replace('1', '+').replace('2', '@')
        return out

    def copy(self):
        return Move(self.final, self.used)

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
        neighbours_of = self.get_neighbours_of()
        moves = []

        # Thanks, alion
        def discover_for(move: Move, tile: int):
            for neighbour in neighbours_of[tile]:
                if neighbour not in move.used:
                    if move not in moves: moves.append(move)
                    move.used.add(neighbour)
                    discover_for(move.copy(), neighbour)
        
        for tile in self.board:
            discover_for(Move(tile, set([tile])), tile)

        return moves

    def make_move(self, move: Move):
        self.board[move.final] *= len(move.used)
        for tile in move.used: 
            if tile != move.final: self.board[tile] = self.new_num()

        self.score += self.board[move.final]

    def show_board(self):
        print('\n')
        for arr in np.split(self.board, 5):
            print(arr)


if __name__ == "__main__":
    game = Gridentify()
    game.show_board()
    moves = game.valid_moves()
    for move in moves[:4]:
        print(move.view())