use rand::prelude::ThreadRng;
use rand::distributions::{Distribution, Uniform};

use crate::moves::Move;

#[derive(Debug)]
pub enum Status {
    Running,
    Ended
}

#[derive(Debug)]
pub struct Game {
    pub board: [u16; 25],
    rng: ThreadRng,
    range: Uniform<u16>,
    pub score: u32,
    pub status: Status,
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
            range,
            score: 0,
            status: Game::check_status(&board),
        }
    }

    pub fn from(board: [u16; 25]) -> Game {
        Game {
            board,
            rng: rand::thread_rng(),
            range: Uniform::new_inclusive(1, 3),
            score: 0,
            status: Game::check_status(&board),
        }
    }

    fn generate_tile(&mut self) -> u16 {
        let x = self.range.sample(&mut self.rng);
        x
    }

    pub fn make_move(&mut self, my_move: Move) {
        let value = self.board[my_move.end];
        let result = value * (1 + my_move.used.len() as u16);
        for pos in my_move.used.into_iter() {
            // verification
            if self.board[pos] != value {panic!("invalid move attempted")}
            // generate new tiles
            self.board[pos] = self.generate_tile();
        }
        self.board[my_move.end] = result;
        self.score += result as u32;
        self.status = Game::check_status(&self.board);
    }

    fn check_status(board: &[u16; 25]) -> Status {
        for i in 0..25 {
            let x = i % 5;
            let y = i / 5;
            let value = board[i];
            if x < 4 && value == board[i + 1] {return Status::Running;} // right
            if y < 4 && value == board[i + 5] {return Status::Running;} // down
            if x > 0 && value == board[i - 1] {return Status::Running;} // left
            if y > 0 && value == board[i - 5] {return Status::Running;} // up
        }
        Status::Ended
    }
}
