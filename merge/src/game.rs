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

    pub fn make_move(&mut self, mov: (usize, usize)) {

    }

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