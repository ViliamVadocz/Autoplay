use crate::error::{Result, UnexpectedVariant};
use array_const_fn_init::array_const_fn_init;
use bitmaps::Bitmap;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use typenum::U25;

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

// these are for black
const fn const_card(num: usize) -> u32 {
    match Card::from_num(num) {
        Card::Tiger => 0b00100_00000_00000_00100_00000,
        Card::Crab => 0b00000_00100_10001_00000_00000,
        Card::Monkey => 0b00000_01010_00000_01010_00000,
        Card::Crane => 0b00000_00100_00000_01010_00000,
        Card::Dragon => 0b00000_10001_00000_01010_00000,
        Card::Elephant => 0b00000_01010_01010_00000_00000,
        Card::Mantis => 0b00000_01010_00000_00100_00000,
        Card::Boar => 0b00000_00100_01010_00000_00000,
        Card::Frog => 0b00000_01000_10000_00010_00000,
        Card::Rabbit => 0b00000_00010_00001_01000_00000,
        Card::Goose => 0b00000_01000_01010_00010_00000,
        Card::Rooster => 0b00000_00010_01010_01000_00000,
        Card::Horse => 0b00000_00100_01000_00100_00000,
        Card::Ox => 0b00000_00100_00010_00100_00000,
        Card::Eel => 0b00000_01000_00010_01000_00000,
        Card::Cobra => 0b00000_00010_01000_00010_00000,
    }
}

const fn const_reversed_card(card: usize) -> u32 {
    const_card(card).reverse_bits() >> 32 - 25
}

const BLACK_CARDS: [u32; 16] = array_const_fn_init![const_card; 16];
const WHITE_CARDS: [u32; 16] = array_const_fn_init![const_reversed_card; 16];

impl Card {
    pub fn get_white(self) -> Bitmap<U25> {
        Bitmap::from_value(WHITE_CARDS[self as usize])
    }

    pub fn get_black(self) -> Bitmap<U25> {
        Bitmap::from_value(BLACK_CARDS[self as usize])
    }

    pub fn is_white(self) -> bool {
        match self {
            Card::Boar => true,
            Card::Cobra => true,
            Card::Crab => false,
            Card::Crane => false,
            Card::Dragon => true,
            Card::Eel => false,
            Card::Elephant => true,
            Card::Frog => true,
            Card::Goose => false,
            Card::Horse => true,
            Card::Mantis => true,
            Card::Monkey => false,
            Card::Ox => false,
            Card::Rabbit => false,
            Card::Rooster => true,
            Card::Tiger => false,
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

    pub fn from_text(text: &str) -> Result<Card> {
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
            _ => Err(Box::new(UnexpectedVariant::new(text.to_string()))),
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

const SHIFT_MASK: [u32; 5] = [
    0b00111_00111_00111_00111_00111,
    0b01111_01111_01111_01111_01111,
    0b11111_11111_11111_11111_11111,
    0b11110_11110_11110_11110_11110,
    0b11100_11100_11100_11100_11100,
];

pub fn shift_bitmap(board: Bitmap<U25>, pos: usize) -> Bitmap<U25> {
    let value = board.into_value();
    let shifted = if pos > 12 {
        value.overflowing_shl(pos as u32 - 12).0
    } else {
        value.overflowing_shr(12 - pos as u32).0
    };
    Bitmap::from_value(shifted & SHIFT_MASK[pos % 5])
}

pub fn print_bitmap(bitmap: &Bitmap<U25>) {
    let mut repr = String::new();
    for index in 0..25 {
        if bitmap.get(index) {
            repr.push('1');
        } else {
            repr.push('0');
        }
        if index % 5 == 4 {
            repr.push('\n')
        } else {
            repr.push(' ')
        }
    }
    println!("{}", repr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_reverse_bitmap() {
        assert_eq!(
            Card::Eel.get_white().into_value(),
            Card::Cobra.get_black().into_value()
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
            let card = test::black_box(Card::Eel.get_white());
            shift_bitmap(card, 6)
        });
    }
}
