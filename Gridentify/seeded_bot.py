from typing import List, Tuple
from copy import deepcopy
import numpy as np

from seeded_grid_game import Move

def new_num(seed) -> Tuple[int, int]:
    """Really bad randomness, same as in the original game."""
    e = (16807 * seed) % 1924421567
    seed = e if e > 0 else e + 3229763266
    num = (e % 3) + 1
    return seed, num


def get_neighbours_of(board: np.ndarray) -> List[int]:
    """Generates a list of neighbours with same value for each tile."""
    neighbours_of = []
    for i, value in enumerate(board):
        neighbours = []
        x = i % 5
        y = i // 5
        if x < 4 and board[i+1] == value: neighbours.append(i+1)
        if y < 4 and board[i+5] == value: neighbours.append(i+5)
        if x > 0 and board[i-1] == value: neighbours.append(i-1)
        if y > 0 and board[i-5] == value: neighbours.append(i-5)
        neighbours_of.append(neighbours)
    return neighbours_of


def gen_valid_moves(board) -> List[Move]:
    """Generates all valid moves on current board."""
    neighbours_of = get_neighbours_of(board)
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
    for i, value in enumerate(board):
        discover_for(Move(i, [i]), i)

    return moves


def simulate_move(board: np.ndarray, seed: int, move: Move) -> Tuple[np.ndarray, int]:
    """Simulates a move in the game."""
    # Sets the final value.
    board[move.final] *= len(move.used)
    # Resets all used tiles except final (in select-order).
    for tile in move.used[::-1]:
        seed, num = new_num(seed)
        if tile != move.final: board[tile] = num
    # Add to score.
    # move_score = board[move.final]
    return board, seed #, move_score


def board_eval(board: np.ndarray) -> int:
    """Static board evaluation."""
    valid_moves = gen_valid_moves(board)
    return len(valid_moves)


def tree_search(board: np.ndarray, seed: int, depth: int) -> Tuple[int, Move]:
    """Recursive tree search to find best move."""

    if depth == 0:
        return board_eval(board), Move(-1)

    else:
        valid_moves = gen_valid_moves(board)
        
        move_evals = []
        for move in valid_moves:
            new_board, new_seed = simulate_move(deepcopy(board), seed, move)
            evaluation, best_move = tree_search(new_board, new_seed, depth - 1)
            move_evals.append(evaluation)

        if len(move_evals) == 0: 
            return 0, Move(-1)

        else:
            best_eval = max(move_evals)
            move_index = move_evals.index(best_eval)

            return best_eval, valid_moves[move_index]