use crate::game::Game;
use crate::card::Card;
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
}
