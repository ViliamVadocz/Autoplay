use crate::board::Board;

enum Status {
    Running,
    Ended
}

enum Player {
    White,
    Red
}

impl Player {
    fn from(index: i64) -> Player {
        if index == 0 {
            Player::White
        } else {
            Player::Red
        }
    }
}

struct Move {
    from: usize,
    to: usize,
}

impl Move {
    fn from(from: usize, to: usize) -> Move {
        Move {from, to}
    }
}

pub struct Game {
    board: Board,
    current_player: Player,
    status: Status,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::White,
            status: Status::Running,
        }
    }

    fn from(board: String, player_index: i64) -> Game {
        Game {
            board: Board::from(board),
            current_player: Player::from(player_index),
            status: Status::Running,
        }
    }

    fn move_gen(&self) -> Vec<Move> {
        let my_pieces = match self.current_player {
            Player::White => &self.board.white,
            Player::Red => &self.board.red
        };

        let mut moves = Vec::new();

        let mut add_moves = |from, x, y, steps| {
            let left = x > steps - 1;
            let right = x < 8 - steps;
            let up = y > steps - 1;
            let down = y < 8 - steps;
            
            if up {
                moves.push(Move::from(from, from - 8 * steps));
                if left {
                    moves.push(Move::from(from, from - 9 * steps));
                }
                if right {
                    moves.push(Move::from(from, from - 7 * steps));
                }
            }

            if left {
                moves.push(Move::from(from, from - steps));
            }
            if right {
                moves.push(Move::from(from, from + steps));
            }

            if down {
                moves.push(Move::from(from, from + 8 * steps));
                if left {
                    moves.push(Move::from(from, from + 7 * steps));
                }
                if right {
                    moves.push(Move::from(from, from + 9 * steps));
                }
            }
        };

        // TODO
        // redo to not give invalid moves (see test below)

        for pos in 0..64 {
            let x = pos % 8;
            let y = pos / 8;
            if my_pieces.get(pos) {
                add_moves(pos, x, y, 1);
                add_moves(pos, x, y, 2);
                if self.board.l2.get(pos) {
                    add_moves(pos, x, y, 3);
                }
            }
        }

        moves
    }

    // pub fn make_move(&mut self, my_move: Move) {
    //     Ok(())
    // }

    fn update_status(&mut self) {
        if self.board.white.len() == 1 || self.board.red.len() == 1 {
            self.status = Status::Ended;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::game::*;

    fn dist(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else{
            b - a
        }
    }
    
    fn validate_moves(game: Game) {
        let moves = game.move_gen();
        let mut first_move = true;
        let mut capture = false;
        for test_move in moves.into_iter() {
            // in bounds
            assert!(0 < test_move.from && test_move.from < 64);
            assert!(0 < test_move.to && test_move.to < 64);
            // to and from are different
            assert_ne!(test_move.from, test_move.to);

            let (my_pieces, enemy_pieces) = match game.current_player {
                Player::White => (&game.board.white, &game.board.red),
                Player::Red => (&game.board.red, &game.board.white),
            };

            // my piece exists at that location
            assert!(my_pieces.get(test_move.from));
            let level_2 = game.board.l2.get(test_move.from);
            
            let from_x = test_move.from % 8;
            let from_y = test_move.from / 8;
            let to_x = test_move.to % 8;
            let to_y = test_move.to / 8;

            let distance: usize;
            if from_y == to_y {
                // horizontal
                distance = dist(from_x, to_x);
            } else if from_x == to_x {
                // vertical
                distance = dist(from_x, to_x)
            } else {
                // diagonal
                let h_dist = dist(from_x, to_x);
                let v_dist = dist(from_y, to_y);
                assert_eq!(h_dist, v_dist);
                distance = h_dist;
            }
            if level_2 {
                assert!(distance < 4);
            } else {
                assert!(distance < 3);
            }

            // forced capture
            let landing_on_opponent = enemy_pieces.get(test_move.to);
            if landing_on_opponent {
                // all moves must be captures if there is a possible capture
                assert!(first_move);
                capture = true;
            }
            
            if capture {
                assert!(landing_on_opponent);
                // can only capture with larger or equal
                if game.board.l2.get(test_move.to) {
                    assert!(level_2)
                }

                // check that enemy is not in the way
                if distance > 1 {
                    let x_dir = if to_x > from_x {1} else if to_x < from_x {-1} else {0};
                    let y_dir = if to_y > from_y {1} else if to_y < from_y {-1} else {0};
                    for steps in 1..(distance as i8) {
                        let pos_x = (from_x as i8 + x_dir * steps) as usize;
                        let pos_y = (from_y as i8 + y_dir * steps) as usize;
                        assert!(!enemy_pieces.get(pos_y * 8 + pos_x));
                    }
                }
            }

            // only level 1s can merge
            else if my_pieces.get(test_move.to) {
                assert!(!level_2);
                assert!(!game.board.l2.get(test_move.to));
            }
            
            if first_move {
                first_move = false;
            }
        }
    }

    #[test]
    fn test_move_gen() {
        validate_moves(Game::from("1010100011010000061000001000000600000606000006000000006600006660".to_string(), 0));
        validate_moves(Game::from("1010100010000000011000001000010600000006000006000000006600006660".to_string(), 1));
        validate_moves(Game::from("1010100010100000010000001000060000000006000006000000060600006660".to_string(), 0));
        validate_moves(Game::from("1000000010100000010000001000000000000006000002000600000600006660".to_string(), 1));
        validate_moves(Game::from("1020000010100000010000001000060000060006000006000000000600006660".to_string(), 0));
        validate_moves(Game::from("1000000010100000000000001000000000010006000000600000020000006060".to_string(), 0));
    }
}
