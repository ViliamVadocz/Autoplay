mod action;
mod bot;
mod card;
mod game;
mod manager;
mod messages;

use std::io::{self, Result, prelude::*};
use crate::{
    action::Action,
    bot::StruggleBot,
    manager::run_bot,
    game::Game
};

#[macro_use]
extern crate serde_derive;


fn main() -> Result<()> {
    let my_bot = MyBot {index: 0, manual: true};
    run_bot(my_bot, "kris6673.synology.me:55445")
}

struct MyBot {
    index: usize,
    manual: bool
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
            // TODO proper strategy other than always drawing
            Ok(Action::Draw(None).to_message())
        }
    }
}

fn get_user_input() -> Result<String> {
    println!("your action: ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!("");
    Ok(buffer)
}
