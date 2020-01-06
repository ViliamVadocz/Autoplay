from typing import List, Tuple
import numpy as np
import numba
from numba import jit

from seeded_grid_game import Move

@jit(nopython=True)
def new_num(seed) -> Tuple[int, int]:
    """Really bad randomness, same as in the original game."""
    e = (16807 * seed) % 1924421567
    seed = e if e > 0 else e + 3229763266
    num = (e % 3) + 1
    return seed, num

@jit(nopython=True)
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


def board_eval1(board: np.ndarray, seed: int) -> int:
    """Static board evaluation. Number of ok moves."""
    valid_moves = gen_valid_moves(board)
    num_ok_moves = 0

    # Simulate every move to see what they would make.
    for move in valid_moves:
        new_board, new_seed = simulate_move(board.copy(), seed, move)
        # Check whether created value is "good"
        if new_board[move.final] not in good_values:
            continue
        else:
            num_ok_moves += 1

    return num_ok_moves

def board_eval2(board: np.ndarray, seed: int) -> int:
    """Static board evaluation."""
    return np.sum(board)

def board_eval3(board: np.ndarray, seed: int) -> int:
    """Static board evaluation."""
    return board_eval2(board, seed) + board_eval1(board, seed)


good_values = set([1,2,3,6,12,24,48,96,192,384,768,1536,3072,6144,12288,24578,49152])
def tree_search(board: np.ndarray, seed: int, depth: int) -> Tuple[int, Move]:
    """Recursive tree search to find best move."""

    if depth == 0:
        return board_eval3(board, seed), Move(-1)

    else:
        valid_moves = gen_valid_moves(board)
        # if len(valid_moves) < 5: print('PANIC')
        
        move_evals = []
        for move in valid_moves:
            new_board, new_seed = simulate_move(board.copy(), seed, move)

            # Check whether created value is "good".
            if len(valid_moves) > 5 and new_board[move.final] not in good_values: # and board.index(max(board)) not in (0, 4, 20, 24)
                evaluation = 0
            else:
                evaluation, best_move = tree_search(new_board, new_seed, depth - 1)


            move_evals.append(evaluation)

        if len(move_evals) == 0: 
            return 0, Move(-1)

        else:
            assert len(valid_moves) == len(move_evals), 'BIG PANIC!'
            best_eval = max(move_evals)
            move_index = move_evals.index(best_eval)

            return best_eval, valid_moves[move_index]