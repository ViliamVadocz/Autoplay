use std::fmt;

use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

use crate::moves::Move;

pub enum Status {
    Running,
    Ended,
}

pub struct Game {
    pub board: [u16; 25],
    pub score: u32,
    pub status: Status,
    rng: ThreadRng,
    range: Uniform<u16>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings = self.board.iter().map(|x| x.to_string());
        let strings_len = strings.clone().map(|x| x.len());
        let longest = strings_len.clone().max().unwrap();
        let padding = strings_len.map(|x| longest - x);
        let mut repr = String::new();
        for (pos, (num, pad)) in strings.zip(padding).enumerate() {
            let padding = String::from(" ").repeat(pad);
            repr += &format!("{} {}", padding, num);
            if pos % 5 == 4 {
                repr.push('\n');
            }
        }
        write!(f, "{}", repr)
    }
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
            score: 0,
            status: Game::check_status(&board),
            rng,
            range,
        }
    }

    pub fn from(board: [u16; 25]) -> Game {
        Game {
            board,
            score: 0,
            status: Game::check_status(&board),
            rng: rand::thread_rng(),
            range: Uniform::new_inclusive(1, 3),
        }
    }

    fn generate_tile(&mut self) -> u16 {
        self.range.sample(&mut self.rng)
    }

    pub fn make_move(&mut self, my_move: Move) {
        let value = self.board[my_move.end];
        let result = value * (1 + my_move.used.len() as u16);
        for pos in 0..25 {
            if my_move.used.get(pos) {
                // verification
                if self.board[pos] != value {
                    panic!("invalid move attempted")
                }
                // generate new tiles
                self.board[pos] = self.generate_tile();
            }
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
            // right
            if x < 4 && value == board[i + 1] {
                return Status::Running;
            }
            // down
            if y < 4 && value == board[i + 5] {
                return Status::Running;
            }
            // left
            if x > 0 && value == board[i - 1] {
                return Status::Running;
            }
            // up
            if y > 0 && value == board[i - 5] {
                return Status::Running;
            }
        }
        Status::Ended
    }
}
