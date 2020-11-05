use bitmaps::Bitmap;
use smallvec::SmallVec;
use typenum::U25;

use crate::cards::{draw_cards, reverse_bitmap, shift_bitmap, Card};
use crate::error::Result;
use crate::messages::*;
use std::fmt;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub used_left_card: bool,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub cards: [Card; 2],
    pub pieces: Bitmap<U25>,
    king: u8,
}

#[derive(Clone, Copy)]
pub struct Game {
    pub white: Player,
    pub black: Player,
    table_card: Card,
    pub white_to_move: bool,
    pub in_progress: bool,
}

impl Game {
    pub fn new() -> Game {
        let cards = draw_cards();
        Game::from_cards(cards)
    }

    pub fn from_cards(mut cards: Vec<Card>) -> Game {
        let last_card = cards.pop().unwrap();
        let white_to_move = last_card.is_white();
        Game {
            white: Player {
                cards: [cards.pop().unwrap(), cards.pop().unwrap()],
                pieces: board!(
                    0 0 0 0 0
                    0 0 0 0 0
                    0 0 0 0 0
                    0 0 0 0 0
                    1 1 1 1 1
                ),
                king: 22,
            },
            black: Player {
                cards: [cards.pop().unwrap(), cards.pop().unwrap()],
                pieces: board!(
                    1 1 1 1 1
                    0 0 0 0 0
                    0 0 0 0 0
                    0 0 0 0 0
                    0 0 0 0 0
                ),
                king: 2,
            },
            table_card: last_card,
            white_to_move,
            in_progress: true,
        }
    }

    pub fn take_turn(&self, my_move: &Move) -> Game {
        let (mut my, mut other, goal) = if self.white_to_move {
            (self.white, self.black, 2)
        } else {
            (self.black, self.white, 22)
        };

        // move my piece
        my.pieces.set(my_move.from as usize, false);
        my.pieces.set(my_move.to as usize, true);
        // move king
        if my.king == my_move.from {
            my.king = my_move.to;
        }
        // remove enemy piece if it is there
        other.pieces.set(my_move.to as usize, false);

        // card management
        let index = !my_move.used_left_card as usize;
        let used_card = my.cards[index];
        my.cards[index] = self.table_card;
        let table_card = used_card;

        // switch turn
        let white_to_move = !self.white_to_move;

        // check for king capture or reaching end
        let in_progress = other.king != my_move.to && my.king != goal;

        let (white, black) = if self.white_to_move {
            (my, other)
        } else {
            (other, my)
        };
        Game {
            white,
            black,
            table_card,
            white_to_move,
            in_progress,
        }
    }

    pub fn gen_moves(&self) -> SmallVec<[Move; 36]> {
        let my = if self.white_to_move {
            &self.white
        } else {
            &self.black
        };

        // get cards
        let mut left = my.cards[0].get_moves();
        let mut right = my.cards[1].get_moves();
        if !self.white_to_move {
            left = reverse_bitmap(&left);
            right = reverse_bitmap(&right);
        }

        let mut moves = SmallVec::new();
        // for every one of my pieces, try each card
        let mut pieces = my.pieces;
        while let Some(from_pos) = pieces.first_index() {
            pieces.set(from_pos, false);
            let mut left_shifted = shift_bitmap(&left, from_pos) & !my.pieces;
            while let Some(to_pos) = left_shifted.first_index() {
                left_shifted.set(to_pos, false);
                moves.push(Move {
                    from: from_pos as u8,
                    to: to_pos as u8,
                    used_left_card: true,
                });
            }
            let mut right_shifted = shift_bitmap(&right, from_pos) & !my.pieces;
            while let Some(to_pos) = right_shifted.first_index() {
                right_shifted.set(to_pos, false);
                moves.push(Move {
                    from: from_pos as u8,
                    to: to_pos as u8,
                    used_left_card: false,
                });
            }
        }
        // if no available moves, you can skip, but you still need to use a card
        if moves.is_empty() {
            moves.push(Move {
                from: my.king,
                to: my.king,
                used_left_card: true,
            });
            moves.push(Move {
                from: my.king,
                to: my.king,
                used_left_card: false,
            });
        }

        moves
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        // colour to move
        if self.white_to_move {
            output.push_str("White to move\n")
        } else {
            output.push_str("Black to move\n")
        }

        // board
        let mut board = String::new();
        for i in 0..25 {
            if self.white.pieces.get(i as usize) {
                if i == self.white.king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if self.black.pieces.get(i as usize) {
                if i == self.black.king {
                    board.push('♚');
                } else {
                    board.push('♟');
                }
            } else {
                board.push('◻');
            }
            board.push(' ');
            if i % 5 == 4 {
                board.push('\n')
            }
        }
        output.push_str(&board);

        // cards
        output.push_str(&format!("Black cards: {:?}\n", self.black.cards));
        output.push_str(&format!("White cards: {:?}\n", self.white.cards));
        output.push_str(&format!("Table card: {:?}\n", self.table_card));

        write!(f, "{}", output)
    }
}

impl Game {
    pub fn from_state_msg(state_msg: StateMsg) -> Result<Game> {
        let mut white = Bitmap::new();
        let mut black = Bitmap::new();
        let mut white_king = 0;
        let mut black_king = 0;
        for (i, character) in (0..25).zip(state_msg.board.chars()) {
            match character {
                '0' => {}
                '1' => {
                    black.set(i as usize, true);
                }
                '2' => {
                    black.set(i as usize, true);
                    black_king = i;
                }
                '3' => {
                    white.set(i as usize, true);
                }
                '4' => {
                    white.set(i as usize, true);
                    white_king = i;
                }
                _ => {}
            };
        }

        let white_to_move = color_is_white(state_msg.current_turn)?;
        let white_cards = [
            Card::from_text(&state_msg.cards.red[0])?,
            Card::from_text(&state_msg.cards.red[1])?,
        ];
        let black_cards = [
            Card::from_text(&state_msg.cards.blue[0])?,
            Card::from_text(&state_msg.cards.blue[1])?,
        ];
        let table_card = Card::from_text(&state_msg.cards.side)?;
        let in_progress = is_in_progress(state_msg.game_state)?;

        Ok(Game {
            white: Player {
                cards: white_cards,
                pieces: white,
                king: white_king,
            },
            black: Player {
                cards: black_cards,
                pieces: black,
                king: black_king,
            },
            table_card,
            white_to_move,
            in_progress,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_gen_moves(b: &mut Bencher) {
        let cards = vec![
            Card::Ox,
            Card::Boar,
            Card::Horse,
            Card::Elephant,
            Card::Crab,
        ];
        let game = test::black_box(Game::from_cards(cards));

        b.iter(|| game.gen_moves());
    }

    #[bench]
    fn bench_take_turn(b: &mut Bencher) {
        let cards = vec![
            Card::Ox,
            Card::Boar,
            Card::Horse,
            Card::Elephant,
            Card::Crab,
        ];
        let game = test::black_box(Game::from_cards(cards));
        let m = test::black_box(game.gen_moves().pop().unwrap());

        b.iter(|| game.take_turn(&m));
    }
}
