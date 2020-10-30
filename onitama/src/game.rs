use bitmaps::Bitmap;
use typenum::U25;

use crate::cards::Card;

struct GameState {
    red: Bitmap<U25>,
    red_king: usize,
    blue: Bitmap<U25>,
    blue_king: usize,
    red_turn: bool,
    red_cards: [Card; 2],
    blue_cards: [Card; 2],
    table_card: Card,
}
