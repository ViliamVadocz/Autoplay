use crate::game::Game;
use crate::card::{Card, CardPlace};
use crate::messages::LastAction;

pub enum Action {
    Draw(Option<Card>),
    Trick(Vec<Card>)
}

impl Action {
    pub fn from(action: &LastAction) -> Action {
        match &action.action_type.as_str() {
            &"draw" => {
                match &action.card {
                    Some(card) => Action::Draw(Some(Card::from(card.to_string()))),
                    None => Action::Draw(None)
                }
            },
            &"play" => Action::Trick(
                action.cards.iter().map(|card| Card::from(card.to_string())).collect()
            ),
            _ => panic!("invalid move type in last move")
        }
    }

    pub fn update(self, game: &mut Game, player_index: usize) {
        let hand = &mut game.players[player_index].hand;

        match self {

            Action::Draw(optional_card) => {
                if let Some(card) = optional_card {
                    // draw from centre
                    hand.push(CardPlace::Known(card));
                } else {
                    // draw from deck
                    hand.push(CardPlace::Unknown);
                }
            }

            Action::Trick(card_vec) => {
                if card_vec.len() == 1 {
                    let Some(card) = card_vec.pop();
                    match card {
                        Card::Joker {id: _} => {
                            // remove joker from unseen cards
                            game.unseen_cards.remove_item(&card);
                            // remove joker from hand
                            remove_card_from_hand(hand, CardPlace::Known(card));                            
                            // joker effect = add three cards to hand
                            hand.push(CardPlace::Unknown);
                            hand.push(CardPlace::Unknown);
                            hand.push(CardPlace::Unknown);
                        }
                        Card::SuitCard {suit: _, value: _} => {
                            panic!("somehow a trick with only one card was played that wasn't a joker");
                        }
                    }
                } else {
                    // cards were probably dropped, so we need to remove any that are new from unseen_cards
                    for card in game.center.iter() {
                        game.unseen_cards.remove_item(card);
                    }
                    // remove used cards from hand
                    for card in card_vec.iter() {
                        game.unseen_cards.remove_item(card);
                        remove_card_from_hand(hand, CardPlace::Known(*card));
                    }
                }
            }
        }
    }
}

fn remove_card_from_hand(hand: &mut Vec<CardPlace>, card: CardPlace) {
    if let None = hand.remove_item(&card) {
        hand.remove_item(&CardPlace::Unknown);
    }
}
