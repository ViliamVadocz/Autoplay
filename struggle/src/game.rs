use crate::messages::GameMessage;

pub struct Game {
    pub players: Vec<PlayerInfo>,
    pub deck_size: u32,
    pub center: Vec<Card>,
    pub cards_seen: Vec<Card>,
    pub current_player_index: usize,
    pub has_moves: bool,
    pub hand: Vec<String> // TODO
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            deck_size: 56,
            center: Vec::new(),
            cards_seen: Vec::new(),
            current_player_index: 0,
            has_moves: true,
            hand: Vec::new()
        }
    }

    pub fn start(&mut self, players: Vec<String>) {
        self.players = players.iter().map(|name| PlayerInfo {name: name.to_string(), hand_size: 2, known_cards: Vec::new()}).collect();
        self.deck_size -= 2 * (self.players.len() as u32);
    }

    pub fn update(&mut self, message: GameMessage) {
        self.hand = message.hand;
        // parse game state
        let game_state = message.game_state;
        self.has_moves = game_state.has_moves;
        self.deck_size = game_state.deck_size;
        self.current_player_index = game_state.current_player;
        // let current_player = &game_state.players[self.current_player_index];
        let previous_player_index = (self.current_player_index + self.players.len() - 1) % self.players.len();
        let previous_player = &game_state.players[previous_player_index];

        // use previous action to update knowledge
        if let Some(previous_action) = &previous_player.last_action {
            println!("{0} played: {1:?}", previous_player.name, previous_action);
        } else {
            println!("{} played: no previous action", previous_player.name);
        }

        println!("---");

        // print centre and deck
        let center = &game_state.center;
        println!("center: {:?}", center);
        let deck_size = game_state.deck_size;
        println!("deck size: {}", deck_size);
    }

    // // get the probability that a card is in the deck
    // fn card_in_deck(self, card: Card) -> f32 {
    //     if card in self.seen {
    //         return 0.0;
    //     } else {
    //         return 1.0 / f32::from(self.deck_size) 
    //     }
    // }
}

pub struct PlayerInfo {
    name: String,
    hand_size: u32,
    known_cards: Vec<Card>
}

enum Suit {
    Club,
    Heart,
    Spade,
    Diamond,
    Joker
}

impl Suit {
    fn from(letter: char) -> Result<Suit, &'static str> {
        match letter {
            'C' => Ok(Suit::Club),
            'H' => Ok(Suit::Heart),
            'S' => Ok(Suit::Spade),
            'D' => Ok(Suit::Diamond),
            'J' => Ok(Suit::Joker),
            _ => Err("unknown suit")
        }
    }
}

pub struct Card {
    value: u32,
    suit: Suit
}

impl Card {
    fn from(card_string: String) -> Card {
        let mut char_iter = card_string.chars();
        let suit_str = char_iter.next().unwrap();
        let suit = Suit::from(suit_str).unwrap();
        let num_str = char_iter.next().unwrap();
        let value = match num_str.to_digit(10) {
            Some(digit) => digit,
            None => match num_str {
                'X' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0
            }
        };
        Card {value, suit}
    }
}