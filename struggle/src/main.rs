mod action;
mod bot;
mod card;
mod game;
mod manager;
mod messages;

use crate::{action::Action, bot::StruggleBot, game::Game, manager::run_bot};
use std::io::{self, prelude::*, Result};

#[macro_use]
extern crate serde_derive;

fn main() -> Result<()> {
    let my_bot = MyBot {
        index: 0,
        manual: true,
    };
    run_bot(my_bot, "kris6673.synology.me:55445")
}

struct MyBot {
    index: usize,
    manual: bool,
}

impl StruggleBot for MyBot {
    fn get_name(&self) -> &str {
        "Will"
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn generate_move(&mut self, game: &Game) -> Result<String> {
        if self.manual {
            get_user_input()
        } else {
            let mut possible_actions =
                Action::possible(&game.my_hand, &game.center, game.deck_size);
            possible_actions.sort_by_cached_key(|action| action.get_score());
            let highest_score = possible_actions.pop().unwrap();
            let possible_draw = possible_actions
                .into_iter()
                .find_map(|action| match action {
                    Action::Draw(_) => Some(action),
                    _ => None,
                });
            Ok(match possible_draw {
                Some(draw) => draw.convert_to_message(),
                None => highest_score.convert_to_message(),
            })
        }
    }
}

fn get_user_input() -> Result<String> {
    println!("your action: ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!();
    Ok(buffer)
}
