use crate::colour::Colour;
use array_const_fn_init::array_const_fn_init;
use bitwise::TestBit;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;
use std::result::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card {
    Boar,
    Cobra,
    Crab,
    Crane,
    Dragon,
    Eel,
    Elephant,
    Frog,
    Goose,
    Horse,
    Mantis,
    Monkey,
    Ox,
    Rabbit,
    Rooster,
    Tiger,
}

// these are for red
#[rustfmt::skip]
const fn const_card(num: usize) -> u32 {
    match Card::from_num(num) {
        Card::Boar =>
            board!(0 0 0 0 0
                   0 0 1 0 0
                   0 1 0 1 0
                   0 0 0 0 0
                   0 0 0 0 0),
        Card::Cobra =>
            board!(0 0 0 0 0
                   0 0 0 1 0
                   0 1 0 0 0
                   0 0 0 1 0
                   0 0 0 0 0),
        Card::Crab =>
            board!(0 0 0 0 0
                   0 0 1 0 0
                   1 0 0 0 1
                   0 0 0 0 0
                   0 0 0 0 0),
        Card::Crane =>
            board!(0 0 0 0 0
                   0 0 1 0 0
                   0 0 0 0 0
                   0 1 0 1 0
                   0 0 0 0 0),
        Card::Dragon =>
            board!(0 0 0 0 0
                   1 0 0 0 1
                   0 0 0 0 0
                   0 1 0 1 0
                   0 0 0 0 0),
        Card::Eel =>
            board!(0 0 0 0 0
                   0 1 0 0 0
                   0 0 0 1 0
                   0 1 0 0 0
                   0 0 0 0 0),
        Card::Elephant =>
            board!(0 0 0 0 0
                   0 1 0 1 0
                   0 1 0 1 0
                   0 0 0 0 0
                   0 0 0 0 0),
        Card::Frog =>
            board!(0 0 0 0 0
                   0 1 0 0 0
                   1 0 0 0 0
                   0 0 0 1 0
                   0 0 0 0 0),
        Card::Goose =>
            board!(0 0 0 0 0
                   0 1 0 0 0
                   0 1 0 1 0
                   0 0 0 1 0
                   0 0 0 0 0),
        Card::Horse =>
            board!(0 0 0 0 0
                   0 0 1 0 0
                   0 1 0 0 0
                   0 0 1 0 0
                   0 0 0 0 0),
        Card::Mantis =>
            board!(0 0 0 0 0
                   0 1 0 1 0
                   0 0 0 0 0
                   0 0 1 0 0
                   0 0 0 0 0),
        Card::Monkey =>
            board!(0 0 0 0 0
                   0 1 0 1 0
                   0 0 0 0 0
                   0 1 0 1 0
                   0 0 0 0 0),
        Card::Ox =>
            board!(0 0 0 0 0
                   0 0 1 0 0
                   0 0 0 1 0
                   0 0 1 0 0
                   0 0 0 0 0),
        Card::Rabbit =>
            board!(0 0 0 0 0
                   0 0 0 1 0
                   0 0 0 0 1
                   0 1 0 0 0
                   0 0 0 0 0),
        Card::Rooster =>
            board!(0 0 0 0 0
                   0 0 0 1 0
                   0 1 0 1 0
                   0 1 0 0 0
                   0 0 0 0 0),
        Card::Tiger =>
            board!(0 0 1 0 0
                   0 0 0 0 0
                   0 0 0 0 0
                   0 0 1 0 0
                   0 0 0 0 0),
    }
}

const fn const_reversed_card(card: usize) -> u32 {
    const_card(card).reverse_bits() >> (32 - 25)
}

const RED_CARDS: [u32; 16] = array_const_fn_init![const_card; 16];
const BLUE_CARDS: [u32; 16] = array_const_fn_init![const_reversed_card; 16];

impl Card {
    pub fn get_move(self, colour: Colour) -> u32 {
        match colour {
            Colour::Blue => BLUE_CARDS[self as usize],
            Colour::Red => RED_CARDS[self as usize],
        }
    }

    pub fn get_colour(self) -> Colour {
        match self {
            Card::Boar => Colour::Red,
            Card::Cobra => Colour::Red,
            Card::Crab => Colour::Blue,
            Card::Crane => Colour::Blue,
            Card::Dragon => Colour::Red,
            Card::Eel => Colour::Blue,
            Card::Elephant => Colour::Red,
            Card::Frog => Colour::Red,
            Card::Goose => Colour::Blue,
            Card::Horse => Colour::Red,
            Card::Mantis => Colour::Red,
            Card::Monkey => Colour::Blue,
            Card::Ox => Colour::Blue,
            Card::Rabbit => Colour::Blue,
            Card::Rooster => Colour::Red,
            Card::Tiger => Colour::Blue,
        }
    }

    pub fn get_name(self) -> &'static str {
        match self {
            Card::Boar => "boar",
            Card::Cobra => "cobra",
            Card::Crab => "crab",
            Card::Crane => "crane",
            Card::Dragon => "dragon",
            Card::Eel => "eel",
            Card::Elephant => "elephant",
            Card::Frog => "frog",
            Card::Goose => "goose",
            Card::Horse => "horse",
            Card::Mantis => "mantis",
            Card::Monkey => "monkey",
            Card::Ox => "ox",
            Card::Rabbit => "rabbit",
            Card::Rooster => "rooster",
            Card::Tiger => "tiger",
        }
    }

    pub fn from_text(text: &str) -> Result<Card, String> {
        match text {
            "boar" => Ok(Card::Boar),
            "cobra" => Ok(Card::Cobra),
            "crab" => Ok(Card::Crab),
            "crane" => Ok(Card::Crane),
            "dragon" => Ok(Card::Dragon),
            "eel" => Ok(Card::Eel),
            "elephant" => Ok(Card::Elephant),
            "frog" => Ok(Card::Frog),
            "goose" => Ok(Card::Goose),
            "horse" => Ok(Card::Horse),
            "mantis" => Ok(Card::Mantis),
            "monkey" => Ok(Card::Monkey),
            "ox" => Ok(Card::Ox),
            "rabbit" => Ok(Card::Rabbit),
            "rooster" => Ok(Card::Rooster),
            "tiger" => Ok(Card::Tiger),
            _ => Err(format!("Unknown card {}", text)),
        }
    }

    pub const fn from_num(num: usize) -> Self {
        match num {
            x if x == Card::Boar as usize => Card::Boar,
            x if x == Card::Cobra as usize => Card::Cobra,
            x if x == Card::Crab as usize => Card::Crab,
            x if x == Card::Crane as usize => Card::Crane,
            x if x == Card::Dragon as usize => Card::Dragon,
            x if x == Card::Eel as usize => Card::Eel,
            x if x == Card::Elephant as usize => Card::Elephant,
            x if x == Card::Frog as usize => Card::Frog,
            x if x == Card::Goose as usize => Card::Goose,
            x if x == Card::Horse as usize => Card::Horse,
            x if x == Card::Mantis as usize => Card::Mantis,
            x if x == Card::Monkey as usize => Card::Monkey,
            x if x == Card::Ox as usize => Card::Ox,
            x if x == Card::Rabbit as usize => Card::Rabbit,
            x if x == Card::Rooster as usize => Card::Rooster,
            x if x == Card::Tiger as usize => Card::Tiger,
            _ => Card::Boar,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::from(self.get_name());
        repr.push('\n');
        let bitmap = self.get_move(Colour::Red);
        for index in 0..25u32 {
            if bitmap.test_bit(index) {
                repr.push('◼');
            } else {
                repr.push('◻');
            }
            if index % 5 == 4 {
                repr.push('\n')
            } else {
                repr.push(' ')
            }
        }
        write!(f, "{}", repr)
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card::from_num(rng.gen_range(0, 16))
    }
}

pub fn draw_cards() -> Vec<Card> {
    let mut drawn = vec![];
    while drawn.len() < 5 {
        let card: Card = rand::random();
        if !drawn.contains(&card) {
            drawn.push(card);
        }
    }
    drawn
}

const fn shift_mask(pos: usize) -> u32 {
    [
        0b00111_00111_00111_00111_00111,
        0b01111_01111_01111_01111_01111,
        0b11111_11111_11111_11111_11111,
        0b11110_11110_11110_11110_11110,
        0b11100_11100_11100_11100_11100,
    ][pos % 5]
}

const SHIFT_MASK: [u32; 25] = array_const_fn_init![shift_mask; 25];

pub fn shift_bitmap(board: u32, pos: u32) -> u32 {
    let shifted = if pos > 12 {
        board.overflowing_shl(pos - 12).0
    } else {
        board.overflowing_shr(12 - pos).0
    };
    shifted & SHIFT_MASK[pos as usize]
}

pub struct BitIter(pub u32);

impl Iterator for BitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let i = self.0.trailing_zeros();
            self.0 &= !(1 << i);
            Some(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_reverse_bitmap() {
        assert_eq!(
            Card::Eel.get_move(Colour::Red),
            Card::Cobra.get_move(Colour::Blue)
        )
    }

    #[test]
    fn test_shift_bitmap() {
        assert_eq!(
            shift_bitmap(
                board!(
                    1 0 0 0 1
                    0 1 0 1 0
                    0 0 1 0 0
                    0 1 0 1 0
                    1 0 0 0 1
                ),
                6
            ),
            board!(
                1 0 1 0 0
                0 1 0 0 0
                1 0 1 0 0
                0 0 0 1 0
                0 0 0 0 0
            )
        )
    }

    #[test]
    fn test_shift_bitmap2() {
        assert_eq!(
            shift_bitmap(
                board!(
                    1 0 0 0 1
                    0 1 0 1 0
                    0 0 1 0 0
                    0 1 0 1 0
                    1 0 0 0 1
                ),
                18
            ),
            board!(
                0 0 0 0 0
                0 1 0 0 0
                0 0 1 0 1
                0 0 0 1 0
                0 0 1 0 1
            )
        )
    }

    #[bench]
    fn bench_shift_bitmap(b: &mut Bencher) {
        b.iter(|| {
            let card = test::black_box(Card::Eel.get_move(Colour::Red));
            shift_bitmap(card, 6)
        });
    }
}
