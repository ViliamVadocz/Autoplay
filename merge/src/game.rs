use bitmaps::Bitmap;
use typenum::U64;


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

/* board layout
0 1 2 3 4 5 6 7
8 9 . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .

w w w w . . . .
w w w . . . . .
w w . . . . . .
w . . . . . . .
. . . . . . . r
. . . . . . r r
. . . . . r r r
. . . . r r r r
*/

struct Board {
    white: Bitmap<U64>,
    red: Bitmap<U64>,
    l2: Bitmap<U64>,
    // l3: Bitmap<U64>,
}

impl Board {
    fn new() -> Board {
        Board {
            white: Bitmap::from_value(16975631),
            red: Bitmap::from_value(17357084619874238464),
            l2: Bitmap::new(),
            // l3: Bitmap::new(),
        }
    }

    fn from(board: String) -> Board {
        let mut white = Bitmap::new();
        let mut red = Bitmap::new();
        let mut l2 = Bitmap::new();
        // let mut l3 = Bitmap::new();

        for (i, s) in board.chars().enumerate() {
            // colour
            match s {
                '1' | '2' | '3' => white.set(i, true),
                '6' | '7' | '8' => red.set(i, true),
                _ => true
            };
            // level
            match s {
                '2' | '7' => l2.set(i, true),
                // '3' | '8' => l3.set(i, true),
                _ => true
            };
        }

        Board {
            white,
            red,
            l2,
            // l3,
        }
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

    fn move_gen(&self) -> Vec<(usize, usize)> {
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
                moves.push((from, from - 8 * steps));
                if left {
                    moves.push((from, from - 9 * steps));
                }
                if right {
                    moves.push((from, from - 7 * steps));
                }
            }

            if left {
                moves.push((from, from - steps));
            }
            if right {
                moves.push((from, from + steps));
            }

            if down {
                moves.push((from, from + 8 * steps));
                if left {
                    moves.push((from, from + 7 * steps));
                }
                if right {
                    moves.push((from, from + 9 * steps));
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

    // pub fn make_move(&mut self, my_move: (usize, usize)) {
    //     Ok(())
    // }

    fn update_status(&mut self) {
        if self.board.white.len() == 1 || self.board.red.len() == 1 {
            self.status = Status::Ended;
        }
    }
}

fn bitmap_to_string(b: Bitmap<U64>) -> String {
    let mut repr = String::new();
    for pos in 0..64 {
        if b.get(pos) {
            repr.push('1');
        } else {
            repr.push('0');
        }
        repr.push(' ');
        if pos % 8 == 7 {
            repr.push('\n');
        }
    }
    repr
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
            assert!(0 < test_move.0 && test_move.0 < 64);
            assert!(0 < test_move.1 && test_move.1 < 64);
            // to and from are different
            assert_ne!(test_move.0, test_move.1);

            let (my_pieces, enemy_pieces) = match game.current_player {
                Player::White => (&game.board.white, &game.board.red),
                Player::Red => (&game.board.red, &game.board.white),
            };

            // my piece exists at that location
            assert!(my_pieces.get(test_move.0));
            let level_2 = game.board.l2.get(test_move.0);
            
            let from_x = test_move.0 % 8;
            let from_y = test_move.0 / 8;
            let to_x = test_move.1 % 8;
            let to_y = test_move.1 / 8;

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
            let landing_on_opponent = enemy_pieces.get(test_move.1);
            if landing_on_opponent {
                // all moves must be captures if there is a possible capture
                assert!(first_move);
                capture = true;
            }
            
            if capture {
                assert!(landing_on_opponent);
                // can only capture with larger or equal
                if game.board.l2.get(test_move.1) {
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
            else if my_pieces.get(test_move.1) {
                assert!(!level_2);
                assert!(!game.board.l2.get(test_move.1));
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
