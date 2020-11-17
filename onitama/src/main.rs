#![feature(test)]
extern crate arrayvec;
extern crate bitwise;
extern crate rand;
extern crate test;
extern crate typenum;
extern crate websocket;
extern crate sdl2;
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
mod gui;
mod messages;
mod perft;

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

When using preset cards they be separated by spaces and in this order:
    [red1] [red2] [blue1] [blue2] [side]";

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            println!("{}", HELP);
            if err.len() > 0 {
                println!("\n{}", err);
            }
        }
    }
}

fn run() -> Result<(), String> {
    let args = cli::parse_args()?;
    gui::run(args)
}
