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
mod error;
mod game;
mod messages;

use crate::bot::get_move;
use crate::error::{Result, UnexpectedMessage};
use crate::game::Game;
use crate::messages::*;
use websocket::client::sync::Client;
use websocket::stream::sync::Stream;
use websocket::{ClientBuilder, Message, OwnedMessage};

const CREATE_MATCH: bool = false;

fn main() -> Result<()> {
    let mut client = ClientBuilder::new("wss://litama.herokuapp.com")?.connect(None)?;
    println!("connected!");

    let match_id: String;
    let token: String;
    let white: bool;
    if CREATE_MATCH {
        // create match
        let create_msg = create_match(&mut client)?;
        match_id = create_msg.match_id;
        token = create_msg.token;
        white = color_is_white(create_msg.color)?;
        println!("match id: {}", match_id);
    } else {
        // join match
        match_id = "5fa051c6c83ffd1ff3e7021d".to_string();
        let join_msg = join_match(&mut client, &match_id)?;
        token = join_msg.token;
        white = color_is_white(join_msg.color)?;
    }

    let state_msg = receive_state(&mut client)?;
    let mut this_game = Game::from_state_msg(state_msg)?;

    while this_game.in_progress {
        println!("{}", this_game);
        // my turn
        if white == this_game.white_to_move {
            let my_move = get_move(&this_game);
            let command = move_to_command(&my_move, &match_id, &token, &this_game);
            send_move(&mut client, command)?;
            this_game.take_turn(&my_move);
        }
        receive_state(&mut client)?;
    }
    Ok(())
}

fn create_match<S: Stream>(client: &mut Client<S>) -> Result<CreateMsg> {
    println!("creating match");
    client.send_message(&Message::text("create"))?;
    match client.recv_message()? {
        OwnedMessage::Text(text) => {
            let message = serde_json::from_str::<LitamaMessage>(&text)?;
            match message {
                LitamaMessage::Create(msg) => Ok(msg),
                _ => Err(Box::new(UnexpectedMessage::new(message))),
            }
        }
        _ => panic!("Didn't receive an OwnedMessage::Text"),
    }
}

fn join_match<S: Stream>(client: &mut Client<S>, match_id: &str) -> Result<JoinMsg> {
    println!("joining match {}", match_id);
    client.send_message(&Message::text(format!("join {}", match_id)))?;
    match client.recv_message()? {
        OwnedMessage::Text(text) => {
            let message = serde_json::from_str::<LitamaMessage>(&text)?;
            match message {
                LitamaMessage::Join(msg) => Ok(msg),
                _ => Err(Box::new(UnexpectedMessage::new(message))),
            }
        }
        _ => panic!("Didn't receive an OwnedMessage::Text"),
    }
}

fn receive_state<S: Stream>(client: &mut Client<S>) -> Result<StateMsg> {
    match client.recv_message()? {
        OwnedMessage::Text(text) => {
            let message = serde_json::from_str::<LitamaMessage>(&text)?;
            match message {
                LitamaMessage::State(msg) => Ok(msg),
                _ => Err(Box::new(UnexpectedMessage::new(message))),
            }
        }
        _ => panic!("Didn't receive an OwnedMessage::Text"),
    }
}

fn send_move<S: Stream>(client: &mut Client<S>, command: String) -> Result<MoveMsg> {
    println!("sending move");
    client.send_message(&Message::text(command))?;
    match client.recv_message()? {
        OwnedMessage::Text(text) => {
            let message = serde_json::from_str::<LitamaMessage>(&text)?;
            match message {
                LitamaMessage::Move(msg) => Ok(msg),
                _ => Err(Box::new(UnexpectedMessage::new(message))),
            }
        }
        _ => panic!("Didn't receive an OwnedMessage::Text"),
    }
}
