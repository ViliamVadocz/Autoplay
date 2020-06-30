use bitmaps::Bitmap;
use typenum::U64;

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

pub struct Board {
    pub white: Bitmap<U64>,
    pub red: Bitmap<U64>,
    pub l2: Bitmap<U64>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: Bitmap::from_value(16975631),
            red: Bitmap::from_value(17357084619874238464),
            l2: Bitmap::new(),
        }
    }

    pub fn from(board: String) -> Board {
        let mut white = Bitmap::new();
        let mut red = Bitmap::new();
        let mut l2 = Bitmap::new();

        for (i, s) in board.chars().enumerate() {
            // colour
            match s {
                '1' | '2' => white.set(i, true),
                '6' | '7' => red.set(i, true),
            };
            // level
            if let '2' | '7' = s {
                l2.set(i, true);
            }
        }

        Board {
            white,
            red,
            l2,
            // l3,
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
