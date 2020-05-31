use std::collections::HashSet;
use crate::game::Game;
use crate::card::Card;
use crate::messages::LastAction;
use crate::card::Suit::*;

pub enum Action {
    Draw(Option<Card>),
    Trick(Vec<Card>)
}

impl Action {
    pub fn from(action: &LastAction) -> Action {
        match action.action_type.as_str() {
            "draw" => match &action.card {
                Some(card) => Action::Draw(Some(Card::from(&card))),
                None => Action::Draw(None)
            },
            "play" => Action::Trick(
                action.cards.iter().map(|card| Card::from(&card)).collect()
            ),
            _ => panic!("invalid action type in last action")
        }
    }

    pub fn update_info(self, game: &mut Game, player_index: usize) {
        let known_hand = &mut game.players[player_index].known_hand;

        match self {
            Action::Draw(optional_card) => {
                if let Some(card) = optional_card {
                    // draw from centre
                    known_hand.insert(card);
                }
            }

            Action::Trick(card_vec) => {
                match card_vec[..] {
                    [Card::Joker {id: _}] => {
                        let card = &card_vec[0];
                        // remove joker from unseen cards
                        game.unseen_cards.remove(card);
                        // remove joker from hand
                        known_hand.remove(card);   
                    }
                    
                    _ => {
                        // cards were probably dropped, so we need to remove any that are new from unseen_cards
                        for card in game.center.iter() {
                            game.unseen_cards.remove(card);
                        }
                        // remove used cards from unseen and hand
                        for card in card_vec.iter() {
                            game.unseen_cards.remove(card);
                            known_hand.remove(card);
                        }
                    }
                }
            }
        }
    }

    pub fn to_message(self) -> String {
        match self {
            Action::Draw(optional_card) => {
                match optional_card {
                    Some(card) => format!("draw {}", card.repr()),
                    None => String::from("draw")
                }
            },
            Action::Trick(cards) => {
                format!("play {}", cards.into_iter().map(|card| card.repr()).collect::<Vec<String>>().join(","))
            }
        }
    }

    pub fn possible(hand: &HashSet<Card>, center: &HashSet<Card>, deck_size: u32) -> Vec<Action> {
        let mut actions = Vec::new();

        // draw from deck if not empty
        if deck_size > 0 {
            actions.push(Action::Draw(None));
        }

        // draw a card from the center
        for &card in center.iter() {
            actions.push(Action::Draw(Some(card)))
        }
        
        let mut club_cards = Vec::new();
        let mut heart_cards = Vec::new();
        let mut spade_cards = Vec::new();
        let mut diamond_cards = Vec::new();
        let mut numeric_tricks: Vec<Vec<Card>> = (1..15).map(|_| Vec::new()).collect();

        // iterate over hand and keep track of possible hands
        for &card in hand.iter() {
            match card {
                Card::SuitCard {suit, value} => {
                    // track possible suit tricks
                    match suit {
                        Club => club_cards.push(card),
                        Heart => heart_cards.push(card),
                        Spade => spade_cards.push(card),
                        Diamond => diamond_cards.push(card)
                    };
                    // track possible numeric tricks
                    numeric_tricks[value as usize].push(card);
                }
                // add any joker tricks
                Card::Joker {id: _} => actions.push(Action::Trick(vec![card]))
            };
        }

        // add whole and valid tricks
        if club_cards.len() > 1 {
            actions.push(Action::Trick(club_cards));
        }
        if heart_cards.len() > 1 {
            actions.push(Action::Trick(heart_cards));
        }
        if spade_cards.len() > 1 {
            actions.push(Action::Trick(spade_cards));
        }
        if diamond_cards.len() > 1 {
            actions.push(Action::Trick(diamond_cards));
        }
        for trick in numeric_tricks.into_iter() {
            if trick.len() > 1 {
                actions.push(Action::Trick(trick));
            }
        }

        actions
    }
}
