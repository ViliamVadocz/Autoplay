#![feature(test)]
extern crate arrayvec;
extern crate bitwise;
extern crate rand;
extern crate test;
extern crate typenum;
extern crate websocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;
mod bot;
mod cards;
mod cli;
mod color;
mod connection;
mod error;
mod game;
mod messages;

use crate::error::Result;

const SERVER: &str = "wss://litama.herokuapp.com";

fn main() -> Result<()> {
    cli::run()
}
