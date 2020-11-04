use bitmaps::Bitmap;
use smallvec::SmallVec;
use typenum::U25;

use crate::cards::{draw_cards, reverse_bitmap, shift_bitmap, Card};
use crate::error::Result;
use crate::messages::*;
use std::fmt;

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub used_left_card: bool,
}

#[derive(Clone)]
pub struct Game {
    white: Bitmap<U25>,
    black: Bitmap<U25>,
    white_king: usize,
    black_king: usize,
    pub white_cards: [Card; 2],
    pub black_cards: [Card; 2],
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
            white: board!(
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                1 1 1 1 1
            ),
            black: board!(
                1 1 1 1 1
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
            ),
            white_king: 22,
            black_king: 2,
            white_cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            black_cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            table_card: last_card,
            white_to_move,
            in_progress: true,
        }
    }

    pub fn take_turn(&mut self, my_move: &Move) {
        // TODO maybe do move validation?

        let (my_board, my_king, my_cards, opp_board, opp_king, goal_pos) = if self.white_to_move {
            (
                &mut self.white,
                &mut self.white_king,
                &mut self.white_cards,
                &mut self.black,
                self.black_king,
                2,
            )
        } else {
            (
                &mut self.black,
                &mut self.black_king,
                &mut self.black_cards,
                &mut self.white,
                self.white_king,
                22,
            )
        };

        // move my piece
        my_board.set(my_move.from, false);
        my_board.set(my_move.to, true);
        // move king
        if *my_king == my_move.from {
            *my_king = my_move.to;
        }
        // remove enemy piece if it is there
        opp_board.set(my_move.to, false);

        // card management
        let (used_card, kept_card) = if my_move.used_left_card {
            (my_cards[0], my_cards[1])
        } else {
            (my_cards[1], my_cards[0])
        };
        *my_cards = [kept_card, self.table_card];
        self.table_card = used_card;

        // switch turn
        self.white_to_move = !self.white_to_move;

        // check for king capture or reaching end
        self.in_progress = opp_king != my_move.to && *my_king != goal_pos;
    }

    pub fn gen_moves(&self) -> SmallVec<[Move; 36]> {
        let (my_cards, my_pieces, my_king) = if self.white_to_move {
            (self.white_cards, self.white, self.white_king)
        } else {
            (self.black_cards, self.black, self.black_king)
        };

        // get cards
        let mut left = my_cards[0].get_moves();
        let mut right = my_cards[1].get_moves();
        if !self.white_to_move {
            left = reverse_bitmap(&left);
            right = reverse_bitmap(&right);
        }

        let mut moves = SmallVec::new();
        // for every one of my pieces, try each card
        let mut pieces = my_pieces;
        while let Some(from_pos) = pieces.first_index() {
            pieces.set(from_pos, false);
            let mut left_shifted = shift_bitmap(&left, from_pos) & !my_pieces;

            while let Some(to_pos) = left_shifted.first_index() {
                left_shifted.set(to_pos, false);
                moves.push(Move {
                    from: from_pos,
                    to: to_pos,
                    used_left_card: true,
                });
            }
            let mut right_shifted = shift_bitmap(&right, from_pos) & !my_pieces;
            while let Some(to_pos) = right_shifted.first_index() {
                right_shifted.set(to_pos, false);
                moves.push(Move {
                    from: from_pos,
                    to: to_pos,
                    used_left_card: false,
                });
            }
        }
        // if no available moves, you can skip, but you still need to use a card
        if moves.is_empty() {
            moves.push(Move {
                from: my_king,
                to: my_king,
                used_left_card: true,
            });
            moves.push(Move {
                from: my_king,
                to: my_king,
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
            if self.white.get(i) {
                if i == self.white_king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if self.black.get(i) {
                if i == self.black_king {
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
        output.push_str(&format!("Black cards: {:?}\n", self.black_cards));
        output.push_str(&format!("White cards: {:?}\n", self.white_cards));
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
                    black.set(i, true);
                }
                '2' => {
                    black.set(i, true);
                    black_king = i;
                }
                '3' => {
                    white.set(i, true);
                }
                '4' => {
                    white.set(i, true);
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
            white,
            black,
            white_king,
            black_king,
            white_cards,
            black_cards,
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
}
