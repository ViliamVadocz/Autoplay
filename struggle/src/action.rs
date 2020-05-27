use crate::game::Game;
use crate::card::{Card, CardPlace};

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

    pub fn update(self, game: Game, player_index: usize) {
        match self {
            Draw(optional_card) => {
                if let Some(card) = optional_card {
                    // draw from centre
                    game.players[player_index].hand.push(CardPlace::Known(card));
                } else {
                    // draw from deck
                    game.players[player_index].hand.push(CardPlace::UnknownCard);
                }
            }
            Trick(card_vec) => {
                if card_vec.len() == 1 {
                    let Some(card) = card_vec.pop();
                    if let Joker(joker) = card {
                        // use joker
                        game.unseen_cards.remove_item(joker);
                        game.players[player_index].hand.push(Card::UnknownCard);
                        game.players[player_index].hand.push(Card::UnknownCard);
                        game.players[player_index].hand.push(Card::UnknownCard);
                        // TODO remove used card from hand
                    }
                } else {
                    // cards were probably dropped, so we need to remove any that are new from unseen_cards
                    for &card in game.centre.iter() {
                        game.unseen_cards.remove_item(card);
                    }
                    // TODO remove used cards from hand
                }
            }
        }
        // TODO
        // if draw
            // update hand (add)
            // if card
                // remove from centre
            // else
                // decrement deck

        // if trick
            // update hand (remove)
            // update unseen cards
            // if joker
                // update hand (add)
            // if suit
                // track suit tricks in player
                // update score?
            // if numeric
                // update score?
    }
}
