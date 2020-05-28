use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Card {
    SuitCard {value: u32, suit: Suit},
    Joker {id: u32}
}

impl Card {
    pub fn from(card_string: String) -> Card {
        // separate card into two characters
        let mut char_iter = card_string.chars();
        let suit_char = char_iter.next().unwrap();
        let num_char = char_iter.next().unwrap();
        // test if joker
        if suit_char == 'J' {
            let id = num_char.to_digit(10).unwrap();
            return Card::Joker {id};
        }
        // otherwise first char is suit
        let suit = Suit::from(suit_char).unwrap();
        let value = match num_char.to_digit(10) {
            Some(digit) => digit,
            None => match num_char {
                'X' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0
            }
        };
        Card::SuitCard {value, suit}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Card::SuitCard {suit, value} => format!("{0} {1}", suit, value),
            Card::Joker {id} => format!("Joker {}", id)
        };
        write!(f, "{}", output)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Suit {
    Club,
    Heart,
    Spade,
    Diamond
}

impl Suit {
    fn from(letter: char) -> Result<Suit, &'static str> {
        match letter {
            'C' => Ok(Suit::Club),
            'H' => Ok(Suit::Heart),
            'S' => Ok(Suit::Spade),
            'D' => Ok(Suit::Diamond),
            _ => Err("unknown suit")
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Suit::Club => "Club",
            Suit::Heart => "Heart",
            Suit::Spade => "Spade",
            Suit::Diamond => "Diamond"
        };
        write!(f, "{}", output)
    }
}
