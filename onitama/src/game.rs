use arrayvec::ArrayVec;
use bitwise::{ClearBit, SetBit, TestBit};

use crate::cards::{draw_cards, shift_bitmap, BitIter, Card};
use crate::color::Color;
use crate::error::Result;
use crate::messages::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub used_left_card: bool,
}

#[derive(Clone)]
pub struct Player {
    pub cards: [Card; 2],
    pub pieces: u32,
    pub king: u8,
}

#[derive(Clone)]
pub struct Game {
    pub my: Player,
    pub other: Player,
    pub table_card: Card,
    pub color: Color,
    pub in_progress: bool,
}

impl Game {
    pub fn new() -> Game {
        let cards = draw_cards();
        Game::from_cards(cards)
    }

    pub fn goal(&self) -> u8 {
        match self.color {
            Color::Blue => 22,
            Color::Red => 2,
        }
    }

    pub fn get_red_blue(&self) -> (&Player, &Player) {
        match self.color {
            Color::Red => (&self.my, &self.other),
            Color::Blue => (&self.other, &self.my),
        }
    }

    pub fn from_cards(mut cards: Vec<Card>) -> Game {
        let table_card = cards.pop().unwrap();
        let color = table_card.get_color();
        let red = Player {
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
        let blue = Player {
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
            Color::Red => (red, blue),
            Color::Blue => (blue, red),
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
        // king capture
        let king_capture = my_move.to == self.other.king;

        let my = Player {
            cards: [self.my.cards[card_index], self.table_card],
            pieces: self.my.pieces.clear_bit(my_move.from).set_bit(my_move.to),
            king: if king_move { my_move.to } else { self.my.king },
        };

        let other = Player {
            cards: self.other.cards,
            pieces: self.other.pieces.clear_bit(my_move.to),
            king: if king_capture { 25 } else { self.other.king }, // move out of board, doesn't get displayed
        };

        // check for king capture or reaching end
        let in_progress = !king_capture && my.king != self.goal();

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
        for from_pos in BitIter(self.my.pieces) {
            let left_shifted = shift_bitmap(left, from_pos) & !self.my.pieces;
            for to_pos in BitIter(left_shifted) {
                moves.push(Move {
                    from: from_pos as u8,
                    to: to_pos as u8,
                    used_left_card: true,
                });
            }
            let right_shifted = shift_bitmap(right, from_pos) & !self.my.pieces;
            for to_pos in BitIter(right_shifted) {
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
        for from_pos in BitIter(self.my.pieces) {
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
        if self.in_progress {
            match self.color {
                Color::Red => output.push_str("Red to move\n"),
                Color::Blue => output.push_str("Blue to move\n"),
            };
        } else {
            match self.color {
                Color::Red => output.push_str("Blue won\n"),
                Color::Blue => output.push_str("Red won\n"),
            };
        }
        let (red, blue) = self.get_red_blue();

        // board
        let mut board = String::new();
        for i in 0..25 {
            if red.pieces.test_bit(i) {
                if i == red.king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if blue.pieces.test_bit(i) {
                if i == blue.king {
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
        output.push_str(&format!("Blue cards: {:?}\n", blue.cards));
        output.push_str(&format!("Red cards: {:?}\n", red.cards));
        output.push_str(&format!("Table card: {:?}\n", self.table_card));

        write!(f, "{}", output)
    }
}

impl Game {
    pub fn from_state_msg(state_msg: StateMsg) -> Result<Game> {
        let mut red = 0u32;
        let mut blue = 0u32;
        let mut red_king = 0;
        let mut blue_king = 0;
        for (i, character) in (0..25).zip(state_msg.board.chars()) {
            match character {
                '0' => {}
                '1' => {
                    blue = blue.set_bit(i);
                }
                '2' => {
                    blue = blue.set_bit(i);
                    blue_king = i;
                }
                '3' => {
                    red = red.set_bit(i);
                }
                '4' => {
                    red = red.set_bit(i);
                    red_king = i;
                }
                _ => {}
            };
        }

        let red_to_move = color_is_red(state_msg.current_turn)?;
        let red_cards = [
            Card::from_text(&state_msg.cards.red[0])?,
            Card::from_text(&state_msg.cards.red[1])?,
        ];
        let blue_cards = [
            Card::from_text(&state_msg.cards.blue[0])?,
            Card::from_text(&state_msg.cards.blue[1])?,
        ];
        let table_card = Card::from_text(&state_msg.cards.side)?;
        let in_progress = is_in_progress(state_msg.game_state)?;

        let red = Player {
            cards: red_cards,
            pieces: red,
            king: red_king,
        };
        let blue = Player {
            cards: blue_cards,
            pieces: blue,
            king: blue_king,
        };
        let (my, other, color) = match red_to_move {
            true => (red, blue, Color::Red),
            false => (blue, red, Color::Blue),
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
