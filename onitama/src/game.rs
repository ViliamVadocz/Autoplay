use arrayvec::ArrayVec;
use bitwise::{ClearBit, SetBit, TestBit};

use crate::cards::{draw_cards, shift_bitmap, Card};
use crate::error::Result;
use crate::messages::*;
use std::fmt;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub used_left_card: bool,
}

#[derive(Clone)]
pub struct Player {
    pub cards: [Card; 2],
    pub pieces: u32,
    king: u8,
}

#[derive(Clone)]
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
        let is_white = self.white_to_move;

        let (my, other, goal) = if is_white {
            (&self.white, &self.black, 2)
        } else {
            (&self.black, &self.white, 22)
        };

        // move king
        let king_move = my.king == my_move.from;
        // card management
        let card_index = my_move.used_left_card as usize;
        let table_card = my.cards[1 - card_index];

        let my = Player {
            cards: [my.cards[card_index], self.table_card],
            pieces: my.pieces.clear_bit(my_move.from).set_bit(my_move.to),
            king: if king_move { my_move.to } else { my.king },
        };

        let other = Player {
            cards: other.cards,
            pieces: other.pieces.clear_bit(my_move.to),
            king: other.king,
        };

        // switch turn
        let white_to_move = !is_white;

        // check for king capture or reaching end
        let in_progress = other.king != my_move.to && my.king != goal;

        let (white, black) = if is_white { (my, other) } else { (other, my) };
        Game {
            white,
            black,
            table_card,
            white_to_move,
            in_progress,
        }
    }

    pub fn gen_moves(&self) -> ArrayVec<[Move; 40]> {
        let (my, left, right) = if self.white_to_move {
            (
                &self.white,
                self.white.cards[0].get_white(),
                self.white.cards[1].get_white(),
            )
        } else {
            (
                &self.black,
                self.black.cards[0].get_black(),
                self.black.cards[1].get_black(),
            )
        };

        let mut moves = ArrayVec::new();
        // for every one of my pieces, try each card
        let mut pieces = my.pieces;
        while pieces != 0 {
            let from_pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(from_pos);
            let mut left_shifted = shift_bitmap(left, from_pos) & !my.pieces;
            while left_shifted != 0 {
                let to_pos = left_shifted.trailing_zeros();
                left_shifted = left_shifted.clear_bit(to_pos);
                moves.push(Move {
                    from: from_pos as u8,
                    to: to_pos as u8,
                    used_left_card: true,
                });
            }
            let mut right_shifted = shift_bitmap(right, from_pos) & !my.pieces;
            while right_shifted != 0 {
                let to_pos = right_shifted.trailing_zeros();
                right_shifted = right_shifted.clear_bit(to_pos);
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

    pub fn count_moves(&self) -> usize {
        let (my, left, right) = if self.white_to_move {
            (
                &self.white,
                self.white.cards[0].get_white(),
                self.white.cards[1].get_white(),
            )
        } else {
            (
                &self.black,
                self.black.cards[0].get_black(),
                self.black.cards[1].get_black(),
            )
        };
        let mut total = 0;

        // for every one of my pieces, try each card
        let mut pieces = my.pieces;
        while pieces != 0 {
            let from_pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(from_pos);
            total += (shift_bitmap(left, from_pos) & !my.pieces).count_ones() as usize;
            total += (shift_bitmap(right, from_pos) & !my.pieces).count_ones() as usize;
        }
        // if no available moves, you can skip, but you still need to use a card
        if total != 0 {
            total
        } else {
            2
        }
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
            if self.white.pieces.test_bit(i) {
                if i == self.white.king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if self.black.pieces.test_bit(i) {
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
        let mut white = 0u32;
        let mut black = 0u32;
        let mut white_king = 0;
        let mut black_king = 0;
        for (i, character) in (0..25).zip(state_msg.board.chars()) {
            match character {
                '0' => {}
                '1' => {
                    black = black.set_bit(i);
                }
                '2' => {
                    black = black.set_bit(i);
                    black_king = i;
                }
                '3' => {
                    white = white.set_bit(i);
                }
                '4' => {
                    white = white.set_bit(i);
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
