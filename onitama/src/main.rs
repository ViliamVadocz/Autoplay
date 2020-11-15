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
mod gui;

const SERVER: &str = "wss://litama.herokuapp.com";
const HELP: &str = "Onitama Interface

Commands:
- help                              :   show this help message
- local random                      :   create a local game with random cards
- local preset [cards]              :   create a local game with preset cards
- online create [username]          :   create an online game
- online join [match id] [username] :   join an online game
- online spectate [match id]        :   spectate an online game

Add the `-h` flag at the end if you want to play instead of the bot
(cards should be separated by spaces)";

fn main() {
    match run() {
        Ok(_) => {},
        Err(err) => {
            println!("{}", HELP);
            if err.len() > 0 {
                println!("\n{}", err);
            }
        },
    }
}

fn run() -> Result<(), String> {
    let args = cli::parse_args()?;
    gui::run(args)
}
