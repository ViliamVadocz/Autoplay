from typing import List, Tuple, Union
import numpy as np
import numba
from numba import jit

from seeded_grid_game import Move

@jit(nopython=True)
def new_num(seed: int) -> Tuple[int, int]:
    """Really bad randomness, same as in the original game."""
    e = (16807 * seed) % 1924421567
    seed = e if e > 0 else e + 3229763266
    num = (e % 3) + 1
    return seed, num

@jit(nopython=True)
def get_neighbours_of(board: np.ndarray) -> List[List[int]]:
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


def gen_valid_moves(board: np.ndarray) -> List[Move]:
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
        if tile != move.final:
            seed, num = new_num(seed)
            board[tile] = num
    # Add to score.
    # move_score = board[move.final]
    return board, seed #, move_score


### EVALUATION FUNCTIONS ###

def eval_num_moves(board: np.ndarray, seed: int) -> int:
    """Static board evaluation. Focuses on number of ok moves."""
    valid_moves = gen_valid_moves(board)
    num_ok_moves = 0

    # Look at every move to see what it would make.
    # Also look at how long it is.
    for move in valid_moves:
        result = board[move.final] * len(move.used)
        # Check whether created value is "good".
        if len(move.used) in good_move_lens and result in good_values:
            num_ok_moves += 1

    return num_ok_moves

def eval_num_moves_old(board: np.ndarray, seed: int) -> int:
    """Static board evaluation. Focuses on number of ok moves."""
    valid_moves = gen_valid_moves(board)
    num_ok_moves = 0

    # Look at every move to see what it would make.
    for move in valid_moves:
        result = board[move.final] * len(move.used)
        # Check whether created value is "good".
        if result in good_values:
            num_ok_moves += 1

    return num_ok_moves

@jit(nopython=True)
def eval_neighbours(board: np.ndarray) -> int:
    """Static board evaluation. Focuses on num of neighbours."""
    summation = 0
    for list_of_neighbours in get_neighbours_of(board):
        summation += len(list_of_neighbours)
    return summation

@jit(nopython=True)
def eval_sum(board: np.ndarray) -> int:
    """Static board evaluation. Focuses on sum of board."""
    return np.sum(board)

@jit(nopython=True)
def eval_corners(board: np.ndarray) -> int:
    """Static board evaluation. Focuses on placing large numbers in the corners."""
    # return board[0] + board[4] + board[20] + board[24]
    return max(board[0], board[4], board[20], board[24])

# Alion's special sauce.
alion_special_sauce = np.array([
        [ 128, 256, 512,1024,2048],
        [  64,  32,  16,   8,   4],
        [   2,   1,   0,   1,   2],
        [   4,   8,  16,  32,  64],
        [2048,1024, 512, 256, 128]
    ])

alion_simple = np.array([
    [ 8, 9,10,11,12],
    [ 7, 6, 5, 4, 3],
    [ 2, 1, 0, 1, 2],
    [ 3, 4, 5, 6, 7],
    [12,11,10, 9, 8]
])
alion_simple = 2 ** alion_simple

one_corner_snail = np.array([
        [ 4, 3, 2, 1, 0],
        [ 5,12,13,14,15],
        [ 6,11,18,17,16],
        [ 7,10,19,22,23],
        [ 8, 9,20,21,24]
    ])
one_corner_snail = 2 ** one_corner_snail
one_corner_snail = one_corner_snail / 4000

weights = alion_special_sauce
# Get rotations and flips of the weights.
a_weights = weights.reshape((25,))
b_weights = np.rot90(weights, 1).reshape((25,))
c_weights = np.rot90(weights, 2).reshape((25,))
d_weights = np.rot90(weights, 3).reshape((25,))
e_weights = np.fliplr(weights).reshape((25,))
f_weights = np.fliplr(np.rot90(weights, 1)).reshape((25,))
g_weights = np.fliplr(np.rot90(weights, 2)).reshape((25,))
h_weights = np.fliplr(np.rot90(weights, 3)).reshape((25,))

@jit(nopython=True)
def eval_scrabble(board: np.ndarray) -> float:
    """Static board evaluation. Uses tile weights to get value of board."""
    a = np.sum(a_weights * board)
    b = np.sum(b_weights * board)
    c = np.sum(c_weights * board)
    d = np.sum(d_weights * board)
    e = np.sum(e_weights * board)
    f = np.sum(f_weights * board)
    g = np.sum(g_weights * board)
    h = np.sum(h_weights * board)
    return max(a, b, c, d, e, f, g, h)

def board_eval(board: np.ndarray, seed: int) -> float:
    """Static board evaluation. Combines various evaluations."""
    nbo = eval_neighbours(board)
    scr = eval_scrabble(board)
    return 100 * nbo*np.log10(scr) + scr

### ###


good_values = set([1,2,3,6,12,24,48,96,192,384,768,1536,3072,6144,12288,24578,49152])
good_move_lens = set([2,3,4,6,8,12,24])
def tree_search(board: np.ndarray, seed: int, depth: int) -> Tuple[float, Union[Move,None]]:
    """Recursive tree search to find best move."""

    if depth == 0:
        return board_eval(board, seed), None

    else:
        valid_moves = gen_valid_moves(board)

        # return negative infinity if board position has no valid moves.
        if len(valid_moves) == 0:
            return np.NINF, None

        move_evals = np.zeros((len(valid_moves),))

        # Panic when the number of possible moves is low.
        # This is just to let it die when the last few moves are 'bad moves'.
        panic = len(valid_moves) < 5
        # if panic: print('PANIC')

        for i, move in enumerate(valid_moves):

            # Prune bad moves if not panicing.
            result = board[move.final] * len(move.used)
            if panic or (len(move.used) in good_move_lens and result in good_values):
                new_board, new_seed = simulate_move(board.copy(), seed, move)
                move_evals[i], best_move = tree_search(new_board, new_seed, depth - 1)
            
            else:
                move_evals[i] = np.NINF

        else:
            move_index = np.argmax(move_evals)
            best_eval = move_evals[move_index]

            return best_eval, valid_moves[move_index]
