use rand::prelude::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Move {
    end: usize,
    used: HashSet<usize>,
}

impl Move {
    fn from(end: usize) -> Move {
        Move {
            end,
            used: HashSet::new(),
        }
    }
}

impl Clone for Move {
    fn clone(&self) -> Move {
        Move {
            end: self.end,
            used: self.used.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub board: [u16; 25],
    rng: ThreadRng,
    range: Uniform<u16>,
}

impl Game {
    pub fn new() -> Game {
        let range = Uniform::new_inclusive(1, 3);
        let mut rng = rand::thread_rng();
        let mut board = [0; 25];
        for tile in board.iter_mut() {
            *tile = range.sample(&mut rng);
        }
        Game {
            board,
            rng,
            range
        }
    }

    pub fn from(board: [u16; 25]) -> Game {
        Game {
            board,
            rng: rand::thread_rng(),
            range: Uniform::new_inclusive(1, 3),
        }
    }

    fn generate_tile(&mut self) -> u16 {
        let x: u16 = self.range.sample(&mut self.rng);
        (x % 3) + 1
    }
    
    pub fn get_neighbours_of(&self) -> Vec<Vec<usize>> {
        let mut neighbours_of = Vec::new();
        for i in 0..25 {
            let x = i % 5;
            let y = i / 5;
            let value = self.board[i];
            let mut neighbours = Vec::new();
            if x < 4 && value == self.board[i + 1] {neighbours.push(i + 1);} // right
            if y < 4 && value == self.board[i + 5] {neighbours.push(i + 5);} // down
            if x > 0 && value == self.board[i - 1] {neighbours.push(i - 1);} // left
            if y > 0 && value == self.board[i - 5] {neighbours.push(i - 5);} // up
            neighbours_of.push(neighbours);
        }
        neighbours_of
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let neighbours_of = self.get_neighbours_of();
        let mut moves = Vec::new();

        // start moves at each tile
        for i in 0..25 {
            explore(&Move::from(i), i, &neighbours_of, &mut moves)
        }

        moves
    }
}

fn explore(branch: &Move, pos: usize, neighbours_of: &Vec<Vec<usize>>, moves: &mut Vec<Move>) {
    // try expanding into each neighbour
    for &neighbour in neighbours_of[pos].iter() {
        // check that this tile is unexplored by this move
        if !(branch.end == neighbour || branch.used.contains(&neighbour)) {
            // branch off
            let mut new_branch = branch.clone();
            new_branch.used.insert(neighbour);
            // recursively explore
            explore(&new_branch, neighbour, neighbours_of, moves);
            moves.push(new_branch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::*;

    #[test]
    fn dead_board() {
        let mut checkerboard = [0; 25];
        for (i, tile) in checkerboard.iter_mut().enumerate() {
            *tile = (i as u16 % 2) + 1
        }
        let game = Game::from(checkerboard);
        let moves = game.possible_moves();
        assert_eq!(moves.len(), 0)
    }

    #[test]
    fn all_ones_board() {
        let game = Game::from([1; 25]);
        let moves = game.possible_moves();
        assert_eq!(moves.len(), 3060392)
    }
}