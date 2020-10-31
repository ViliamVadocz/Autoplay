use bitmaps::Bitmap;
use typenum::U25;

use std::fmt;

use crate::cards::{Card, draw_cards, reverse_bitmap, shift_bitmap};

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub used_left_card: bool,
}

pub struct Game {
    white: Bitmap<U25>,
    black: Bitmap<U25>,
    white_king: usize,
    black_king: usize,
    white_cards: [Card; 2],
    black_cards: [Card; 2],
    table_card: Card,
    white_to_move: bool,
    pub in_progress: bool,
}

impl Game {
    pub fn new() -> Game {
        // TODO: Generate cards
        let mut cards = draw_cards();
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

    pub fn take_turn(&mut self, my_move: Move) {
        // TODO maybe do move validation?

        let (my_board, my_king, my_cards, opp_board, opp_king, goal_pos) = if self.white_to_move {
            (&mut self.white, &mut self.white_king, &mut self.white_cards, &mut self.black, self.black_king, 2)
        } else {
            (&mut self.black, &mut self.black_king, &mut self.black_cards, &mut self.white, self.white_king, 22)
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

    pub fn gen_moves(&self) -> Vec<Move> {
        let (my_cards, my_pieces, my_king) = if self.white_to_move {
            (&self.white_cards, &self.white, self.white_king)
        } else {
            (&self.black_cards, &self.black, self.black_king)
        };

        // get cards
        let mut left = my_cards[0].get_moves();
        let mut right = my_cards[1].get_moves();
        if !self.white_to_move {
            left = reverse_bitmap(&left);
            right = reverse_bitmap(&right);
        }
        
        let mut moves = Vec::new();
        // for every one of my pieces, try each card
        for from_pos in 0..25 {
            if my_pieces.get(from_pos) {
                let left_shifted = shift_bitmap(&left, from_pos);
                let right_shifted = shift_bitmap(&right, from_pos);
                for to_pos in 0..25 {
                    // cannot go to a position already occupied by my piece
                    if !my_pieces.get(to_pos) {
                        if left_shifted.get(to_pos) {
                            moves.push(Move {from: from_pos, to: to_pos, used_left_card: true});
                        }
                        if right_shifted.get(to_pos) {
                            moves.push(Move {from: from_pos, to: to_pos, used_left_card: false});
                        }
                    }
                }
            }
        }
        // if no available moves, you can skip, but you still need to use a card
        if moves.is_empty() {
            moves.push(Move {from: my_king, to: my_king, used_left_card: true});
            moves.push(Move {from: my_king, to: my_king, used_left_card: false});
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