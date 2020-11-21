#![feature(test)]
extern crate arrayvec;
extern crate bitwise;
extern crate rand;
extern crate sdl2;
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
mod colour;
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
            if !err.is_empty() {
                println!("Err: {}\n", err);
            }
            println!("{}", HELP);
        }
    }
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use crate::bot::get_move;
use crate::cli::Args;
use crate::cli::{GameHost, Playing};
use crate::colour::Colour;
use crate::connection::{Connection, Participant};
use crate::game::{Game, Move};

pub enum Transmission {
    Display(Game),
    Usernames(String, String),
    RequestMove,
}

fn run() -> Result<(), String> {
    let args = cli::parse_args()?;

    // track whether the program should exit
    let should_end = Arc::new(AtomicBool::new(false));

    // communication between game and gui
    let (tx_gui, rx_game) = channel();
    let (tx_game, rx_gui) = channel();

    // TODO no-gui option
    let gui_should_end = Arc::clone(&should_end);
    let gui_thread = thread::spawn(move || {
        let res = gui::run(tx_gui, rx_gui, &gui_should_end);
        gui_should_end.store(true, Ordering::Relaxed);
        res
    });

    let game_should_end = Arc::clone(&should_end);
    let game_thread = thread::spawn(move || {
        let res = run_game(tx_game, rx_game, args, &game_should_end);
        if res.is_err() {
            game_should_end.store(true, Ordering::Relaxed);
        }
        res
    });

    loop {
        if should_end.load(Ordering::Relaxed) {
            let gui_res = gui_thread.join().unwrap();
            let game_res = game_thread.join().unwrap();
            return match &gui_res {
                Ok(_) => game_res,
                Err(gui_err) => match &game_res {
                    Ok(_) => gui_res,
                    Err(game_err) => Err(format!("{}\n{}", gui_err, game_err)),
                },
            };
        }
    }
}

fn run_game(
    tx_game: Sender<Transmission>,
    rx_game: Receiver<Move>,
    args: Args,
    should_end: &Arc<AtomicBool>,
) -> Result<(), String> {
    // helper closures
    let display = |game: &Game| {
        // println!("{}", game);
        tx_game
            .send(Transmission::Display(game.clone()))
            .map_err(|e| e.to_string())
    };
    let get_move_from_gui = || {
        tx_game
            .send(Transmission::RequestMove)
            .map_err(|e| e.to_string())?;
        rx_game.recv().map_err(|e| e.to_string())
    };
    let send_usernames = |red: &str, blue: &str| {
        tx_game
            .send(Transmission::Usernames(red.to_string(), blue.to_string()))
            .map_err(|e| e.to_string())
    };

    let (playing, host) = args;
    match host {
        GameHost::Local(mut game) => {
            let my_colour = Colour::Red; // TODO pick randomly?
            while game.in_progress {
                if should_end.load(Ordering::Relaxed) {
                    break;
                }
                display(&game)?;
                let the_move = if my_colour == game.colour {
                    // my turn
                    match playing {
                        Playing::Human => get_move_from_gui()?,
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    }
                } else {
                    // otherwise bot plays
                    get_move(&game)
                };
                game = game.take_turn(&the_move);
            }
            display(&game)?;
        }

        GameHost::Online(maybe_match_id, username) => {
            let mut conn = Connection::new(SERVER)?;

            let (match_id, p) = match maybe_match_id {
                Some(match_id) => {
                    let p = if matches!(playing, Playing::No) {
                        // fake participant
                        Participant {
                            token: String::new(),
                            index: 0,
                        }
                    } else {
                        conn.join_match(&match_id, &username)
                    };
                    (match_id, p)
                }
                None => conn.create_match(&username),
            };
            println!("match id: {}", match_id);
            // println!("join: https://git.io/onitama#{}", match_id);
            // println!("spectate: https://git.io/onitama#spectate-{}", match_id);

            let mut state_msg = conn.spectate(&match_id);
            let colour = if p.index == state_msg.indices.red {
                Colour::Red
            } else {
                Colour::Blue
            };
            send_usernames(&state_msg.usernames.red, &state_msg.usernames.blue)?;
            let mut game = Game::from_state_msg(state_msg);
            while game.in_progress {
                if should_end.load(Ordering::Relaxed) {
                    break;
                }
                display(&game)?;
                if colour == game.colour && !matches!(playing, Playing::No) {
                    let my_move = match playing {
                        Playing::Human => get_move_from_gui()?,
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    };
                    state_msg = conn.make_move(&my_move, &match_id, &p.token, &game);
                } else {
                    state_msg = conn.recv_state();
                }
                game = Game::from_state_msg(state_msg);
            }
            display(&game)?;
        }
    };
    Ok(())
}
