use crate::bot::get_move;
use crate::cards::Card;
use crate::connection::{Connection, Participant};
use crate::error::Result;
use crate::game::{Game, Move};
use crate::messages::{move_to_command, translate_pos_back};
use crate::SERVER;

use std::fmt;
use std::io::Write;
use std::io::{stdin, stdout};

// TODO add colours with https://docs.rs/ansi_term/0.12.1/ansi_term/

pub fn run() -> Result<()> {
    let manual = choice("manual", "bot")?;
    let online = choice("online", "local")?;

    if online {
        // connect to server
        let mut conn = Connection::new(SERVER)?;
        let create_new = choice("create", "join")?;

        // get the token, colour, and match_id
        let p: Participant;
        let match_id: String;
        if create_new {
            // create a match
            let tup = conn.create_match()?;
            match_id = tup.0;
            p = tup.1;
            println!("match id: {}", match_id);
        } else {
            // join a match
            match_id = input(&"match id:\n> ")?.trim().to_string();
            p = conn.join_match(&match_id)?; // TODO ask again for match id if error
        }
        let mut game = Game::from_state_msg(conn.recv_state()?)?;
        println!(
            "This game's cards:\n{}{}{}{}{}",
            game.white.cards[0],
            game.white.cards[1],
            game.black.cards[0],
            game.black.cards[1],
            game.table_card
        );
        println!(
            "You are playing as {}.",
            if p.white { "WHITE" } else { "BLACK" }
        );
        while game.in_progress {
            println!("{}", game);
            if game.white_to_move == p.white {
                let my_move = if manual {
                    get_move_input(&game)?
                } else {
                    get_move(&game)
                };
                conn.send(&move_to_command(&my_move, &match_id, &p.token, &game))?;
                println!("{:#?}", conn.recv()?);
            }
            game = Game::from_state_msg(conn.recv_state()?)?;
        }
    } else {
        let white = choice("white", "black")?;
        let random = choice("random", "preset")?;
        if random {
            // TODO random cards
            println!("generating random cards")
        } else {
            // TODO ask for cards
            println!("enter cards..............")
        }
        // TODO play the game
        println!("game game game")
    }
    Ok(())
}

fn get_move_input(game: &Game) -> Result<Move> {
    loop {
        let ans = input(&"enter move (format: [card] [from] [to])\n> ")?.to_lowercase();
        let mut words: Vec<&str> = ans.split_whitespace().collect();
        let num_words = words.len();

        if num_words != 3 {
            println!("expected three words, got {}", num_words);
            continue;
        }
        // get positions
        let to_text = words.pop().unwrap();
        let from_text = words.pop().unwrap();
        let to = match translate_pos_back(to_text) {
            Ok(pos) => pos,
            Err(_) => {
                println!("{} is not a valid position", to_text);
                continue;
            }
        } as u8;
        let from = match translate_pos_back(from_text) {
            Ok(pos) => pos,
            Err(_) => {
                println!("{} is not a valid position", from_text);
                continue;
            }
        } as u8;
        // get card
        let card_text = words.pop().unwrap();
        let card_result = Card::from_text(&card_text);
        if card_result.is_err() {
            println!("unknown card {}", &card_text);
            continue;
        }
        let card = card_result.unwrap();
        // check if card in hand
        let my = if game.white_to_move {
            &game.white
        } else {
            &game.black
        };
        if !my.cards.iter().any(|&c| c == card) {
            println!("{} is not in your hand", card);
            continue;
        }
        // check if valid move
        let the_move = Move {
            from,
            to,
            used_left_card: my.cards[0] == card,
        };
        let moves = game.gen_moves();
        if moves.contains(&the_move) {
            return Ok(the_move);
        } else {
            println!("that is not a valid move");
        }
    }
}

fn choice(option_a: &str, option_b: &str) -> Result<bool> {
    let question = format!("[ {} | {} ]\n> ", option_a, option_b);
    loop {
        let ans = input(&question)?.to_lowercase();
        if ans.contains(option_a) {
            return Ok(true);
        } else if ans.contains(option_b) {
            return Ok(false);
        } else {
            println!("⚠️  that's not one of the options");
        }
    }
}

fn input(message: &'_ impl fmt::Display) -> Result<String> {
    print!("{}", message);
    stdout().flush()?;
    let mut ret = String::new();
    stdin().read_line(&mut ret)?;
    Ok(ret)
}
