use crate::error::{Result, UnexpectedVariant};
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

impl Card {
    #[rustfmt::skip]
    pub fn get_moves(&self) -> Bitmap<U25> {
        match self {
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

    pub fn is_white(&self) -> bool {
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

    pub fn get_name(&self) -> &str {
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
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        match rng.gen_range(0, 16) {
            0 => Card::Boar,
            1 => Card::Cobra,
            2 => Card::Crab,
            3 => Card::Crane,
            4 => Card::Dragon,
            5 => Card::Eel,
            6 => Card::Elephant,
            7 => Card::Frog,
            8 => Card::Goose,
            9 => Card::Horse,
            10 => Card::Mantis,
            11 => Card::Monkey,
            12 => Card::Ox,
            13 => Card::Rabbit,
            14 => Card::Rooster,
            _ => Card::Tiger,
        }
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

pub fn reverse_bitmap(board: &Bitmap<U25>) -> Bitmap<U25> {
    let value = board.clone().into_value();
    let reversed = value.reverse_bits() >> 32 - 25;
    Bitmap::from_value(reversed)
}

const SHIFT_MASK: [u32; 5] = [
    0b00111_00111_00111_00111_00111,
    0b01111_01111_01111_01111_01111,
    0b11111_11111_11111_11111_11111,
    0b11110_11110_11110_11110_11110,
    0b11100_11100_11100_11100_11100,
];

pub fn shift_bitmap(board: &Bitmap<U25>, pos: usize) -> Bitmap<U25> {
    let value = board.clone().into_value();
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
            reverse_bitmap(&Card::Eel.get_moves()),
            Card::Cobra.get_moves()
        )
    }

    #[bench]
    fn bench_reverse_bitmap(b: &mut Bencher) {
        b.iter(|| {
            let card = test::black_box(Card::Eel.get_moves());
            reverse_bitmap(&card)
        });
    }

    #[test]
    fn test_shift_bitmap() {
        assert_eq!(
            shift_bitmap(
                &board!(
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
                &board!(
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
            let card = test::black_box(Card::Eel.get_moves());
            shift_bitmap(&card, 6)
        });
    }
}
