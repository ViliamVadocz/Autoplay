use bitmaps::Bitmap;
use typenum::U25;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

pub fn reverse(board: &Bitmap<U25>) -> Bitmap<U25> {
    let mut reversed = Bitmap::new();
    for index in 0..25 {
        if board.get(index) {
            reversed.set(25 - index, true);
        }
    }
    reversed
}
