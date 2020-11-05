#![feature(test)]
extern crate bitmaps;
extern crate rand;
extern crate smallvec;
extern crate test;
extern crate typenum;
extern crate websocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;
mod bot;
mod cards;
mod connection;
mod error;
mod game;
mod messages;

use crate::bot::get_move;
use crate::connection::Connection;
use crate::error::Result;
use crate::game::Game;
use crate::messages::move_to_command;

fn main() -> Result<()> {
    let mut conn1 = Connection::new("wss://litama.herokuapp.com")?;
    let (match_id, p1) = conn1.create_match()?;
    println!("match id: {}", match_id);

    let mut conn2 = Connection::new("wss://litama.herokuapp.com")?;
    let p2 = conn2.join_match(&match_id)?;

    conn1.recv_state()?; // ignore one because we get two
    let state_msg = conn2.recv_state()?;
    let mut this_game = Game::from_state_msg(state_msg)?;

    while this_game.in_progress {
        println!("{}", this_game);
        // TEMP same bot for both sides
        let my_move = get_move(this_game);

        if this_game.white_to_move == p1.white {
            conn1.send(&move_to_command(&my_move, &match_id, &p1.token, &this_game))?;
            println!("{:#?}", conn1.recv()?);
            println!("{:#?}", conn1.recv()?);
        } else {
            conn2.send(&move_to_command(&my_move, &match_id, &p2.token, &this_game))?;
            println!("{:#?}", conn2.recv()?);
            println!("{:#?}", conn2.recv()?);
        }

        this_game = this_game.take_turn(&my_move);
    }

    Ok(())
}
