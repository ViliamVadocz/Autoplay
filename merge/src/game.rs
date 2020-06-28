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
    l3: Bitmap<U64>,
}

impl Board {
    fn new() -> Board {
        Board {
            white: Bitmap::from_value(16975631),
            red: Bitmap::from_value(17357084619874238464),
            l2: Bitmap::new(),
            l3: Bitmap::new(),
        }
    }

    fn from(board: String) -> Board {
        let mut white = Bitmap::new();
        let mut red = Bitmap::new();
        let mut l2 = Bitmap::new();
        let mut l3 = Bitmap::new();

        for (i, s) in board.chars().enumerate() {
            // colour
            match s {
                '1' | '2' | '3' => white.set(i, true),
                '6' | '7' | '8' => red.set(i, true),
            };
            // level
            match s {
                '2' | '7' => l2.set(i, true),
                '3' | '8' => l3.set(i, true),
            };
        }

        Board {
            white,
            red,
            l2,
            l3,
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

    // fn move_gen(&self) {
    //     for pos in 0..64 {
    //     }
    // }

    // pub fn make_move(&mut self, mov: (u8, u8)) {

    // }
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