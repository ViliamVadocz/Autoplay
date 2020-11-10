use arrayvec::ArrayVec;
use bitwise::{ClearBit, SetBit, TestBit};

use crate::cards::{draw_cards, shift_bitmap, BitIter, Card};
use crate::color::Color;
use crate::error::Result;
use crate::messages::*;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    pub my: Player,
    pub other: Player,
    pub table_card: Card,
    pub color: Color,
    pub in_progress: bool,
}

impl Color {
    fn goal(self) -> u8 {
        match self {
            Color::Black => 22,
            Color::White => 2,
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let cards = draw_cards();
        Game::from_cards(cards)
    }

    pub fn get_white_black(&self) -> (&Player, &Player) {
        match self.color {
            Color::White => (&self.my, &self.other),
            Color::Black => (&self.other, &self.my),
        }
    }

    pub fn from_cards(mut cards: Vec<Card>) -> Game {
        let table_card = cards.pop().unwrap();
        let color = table_card.get_color();
        let white = Player {
            cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            pieces: board!(
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                1 1 1 1 1
            ),
            king: 22,
        };
        let black = Player {
            cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            pieces: board!(
                1 1 1 1 1
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
            ),
            king: 2,
        };
        let (my, other) = match color {
            Color::White => (white, black),
            Color::Black => (black, white),
        };
        Game {
            my,
            other,
            table_card,
            color,
            in_progress: true,
        }
    }

    pub fn take_turn(&self, my_move: &Move) -> Game {
        // move king
        let king_move = self.my.king == my_move.from;
        // card management
        let card_index = my_move.used_left_card as usize;
        let table_card = self.my.cards[1 - card_index];

        let my = Player {
            cards: [self.my.cards[card_index], self.table_card],
            pieces: self.my.pieces.clear_bit(my_move.from).set_bit(my_move.to),
            king: if king_move { my_move.to } else { self.my.king },
        };

        let other = Player {
            cards: self.other.cards,
            pieces: self.other.pieces.clear_bit(my_move.to),
            king: self.other.king,
        };

        // check for king capture or reaching end
        let in_progress = other.king != my_move.to && my.king != self.color.goal();

        Game {
            my: other,
            other: my,
            table_card,
            color: self.color.next(),
            in_progress,
        }
    }

    pub fn gen_moves(&self) -> ArrayVec<[Move; 40]> {
        let left = self.my.cards[0].get_move(self.color);
        let right = self.my.cards[1].get_move(self.color);
        let mut moves = ArrayVec::new();
        // for every one of my pieces, try each card
        for (from_pos, _) in BitIter(self.my.pieces) {
            let left_shifted = shift_bitmap(left, from_pos) & !self.my.pieces;
            for (to_pos, _) in BitIter(left_shifted) {
                moves.push(Move {
                    from: from_pos as u8,
                    to: to_pos as u8,
                    used_left_card: true,
                });
            }
            let right_shifted = shift_bitmap(right, from_pos) & !self.my.pieces;
            for (to_pos, _) in BitIter(right_shifted) {
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
                from: self.my.king,
                to: self.my.king,
                used_left_card: true,
            });
            moves.push(Move {
                from: self.my.king,
                to: self.my.king,
                used_left_card: false,
            });
        }

        moves
    }

    pub fn count_moves(&self) -> usize {
        let left = self.my.cards[0].get_move(self.color);
        let right = self.my.cards[1].get_move(self.color);
        let mut total = 0;

        // for every one of my pieces, try each card
        for (from_pos, _) in BitIter(self.my.pieces) {
            total += (shift_bitmap(left, from_pos) & !self.my.pieces).count_ones() as usize;
            total += (shift_bitmap(right, from_pos) & !self.my.pieces).count_ones() as usize;
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
        match self.color {
            Color::White => output.push_str("White to move\n"),
            Color::Black => output.push_str("White to move\n"),
        };

        let (white, black) = self.get_white_black();

        // board
        let mut board = String::new();
        for i in 0..25 {
            if white.pieces.test_bit(i) {
                if i == white.king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if black.pieces.test_bit(i) {
                if i == black.king {
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
        output.push_str(&format!("Black cards: {:?}\n", black.cards));
        output.push_str(&format!("White cards: {:?}\n", white.cards));
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

        let white = Player {
            cards: white_cards,
            pieces: white,
            king: white_king,
        };
        let black = Player {
            cards: black_cards,
            pieces: black,
            king: black_king,
        };
        let (my, other, color) = match white_to_move {
            true => (white, black, Color::White),
            false => (black, white, Color::Black),
        };
        Ok(Game {
            my,
            other,
            table_card,
            color,
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
