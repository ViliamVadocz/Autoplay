use std::fmt;

use bitmaps::Bitmap;
use typenum::U25;

#[derive(Clone)]
pub struct Move {
    pub end: usize,
    pub used: Bitmap<U25>,
}

impl Move {
    fn from(end: usize) -> Move {
        Move {
            end,
            used: Bitmap::new(),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for pos in 0..25 {
            repr.push(' ');
            if pos == self.end {
                repr.push('X');
            } else if self.used.get(pos) {
                repr.push('*');
            } else {
                repr.push('.');
            }
            if pos % 5 == 4 {
                repr.push('\n');
            }
        }
        write!(f, "{}", repr)
    }
}

fn get_neighbours_of(board: &[u16; 25]) -> Vec<Vec<usize>> {
    let mut neighbours_of = Vec::new();
    for i in 0..25 {
        let x = i % 5;
        let y = i / 5;
        let value = board[i];
        // 0 never appears so we use it for off-limits tiles
        if value == 0 {
            neighbours_of.push(Vec::new());
            continue;
        }
        let mut neighbours = Vec::new();
        // right
        if x < 4 && value == board[i + 1] {
            neighbours.push(i + 1);
        }
        // down
        if y < 4 && value == board[i + 5] {
            neighbours.push(i + 5);
        }
        // left
        if x > 0 && value == board[i - 1] {
            neighbours.push(i - 1);
        }
        // up
        if y > 0 && value == board[i - 5] {
            neighbours.push(i - 5);
        }
        neighbours_of.push(neighbours);
    }
    neighbours_of
}

pub fn possible_moves(board: &[u16; 25]) -> Vec<Move> {
    let neighbours_of = get_neighbours_of(board);
    let mut moves = Vec::new();

    // start moves at each tile
    for i in 0..25 {
        if !neighbours_of[i].is_empty() {
            explore(&Move::from(i), i, &neighbours_of, &mut moves)
        }
    }

    moves
}

fn explore(branch: &Move, pos: usize, neighbours_of: &[Vec<usize>], moves: &mut Vec<Move>) {
    // try expanding into each neighbour
    for &neighbour in neighbours_of[pos].iter() {
        // check that this tile is unexplored by this move
        if !(branch.end == neighbour || branch.used.get(neighbour)) {
            // branch off
            let mut new_branch = branch.clone();
            new_branch.used.set(neighbour, true);
            // recursively explore
            explore(&new_branch, neighbour, neighbours_of, moves);
            moves.push(new_branch);
        }
    }
}

pub fn possible_boards(mut board: [u16; 25], my_move: &Move) -> Vec<[u16; 25]> {
    let value = board[my_move.end];
    let result = value * (1 + my_move.used.len() as u16);
    let n = my_move.used.len() as u32;
    // could fail if a move of length > 20 arrived
    board[my_move.end] = result;
    (0..3_u32.pow(n))
        .map(|i| {
            let mut new_board = board;
            let mut values = (0..n).map(|x| i / 3_u32.pow(x) % 3 + 1);
            for (pos, tile) in new_board.iter_mut().enumerate() {
                if my_move.used.get(pos) {
                    *tile = values.next().unwrap() as u16;
                }
            }
            new_board
        })
        .collect()
}

/// make move, but place marker zeroes where newly generated tiles would be to make them off-limits
pub fn get_fake_board(mut board: [u16; 25], my_move: &Move) -> [u16; 25] {
    let value = board[my_move.end];
    let result = value * (1 + my_move.used.len() as u16);
    board[my_move.end] = result;
    for (pos, tile) in board.iter_mut().enumerate() {
        if my_move.used.get(pos) {
            *tile = 0;
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use crate::moves::{possible_boards, possible_moves, Move};

    #[test]
    fn dead_board() {
        let mut chequerboard = [0; 25];
        for (i, tile) in chequerboard.iter_mut().enumerate() {
            *tile = (i as u16 % 2) + 1;
        }
        let moves = possible_moves(&chequerboard);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn layers() {
        let mut layers = [0; 25];
        for (i, tile) in layers.iter_mut().enumerate() {
            let layer = i as u16 / 5;
            *tile = (layer % 3) + 1;
        }
        let moves = possible_moves(&layers);
        assert_eq!(moves.len(), 100);
    }

    #[test]
    fn all_ones_board() {
        let moves = possible_moves(&[1; 25]);
        assert_eq!(moves.len(), 3060392);
    }

    #[test]
    fn possible_board_test() {
        let board = [1; 25];
        let mut my_move = Move::from(0);
        let num_covered = 10;
        for i in 1..num_covered {
            my_move.used.set(i, true);
        }
        let boards = possible_boards(board, &my_move);
        assert_eq!(boards.len(), 3_usize.pow((num_covered - 1) as u32));
    }
}
