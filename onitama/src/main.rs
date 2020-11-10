#![feature(test)]
extern crate arrayvec;
extern crate bitwise;
extern crate rand;
extern crate test;
extern crate typenum;
extern crate websocket;
// extern crate sdl2;
#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;
mod bot;
mod cards;
mod cli;
mod color;
mod connection;
mod game;
mod messages;
mod perft;

const SERVER: &str = "wss://litama.herokuapp.com";

fn main() {
    cli::run()
}
