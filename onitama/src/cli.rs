use crate::bot::{get_move, get_move_hytak};
use crate::cards::Card;
use crate::color::Color;
use crate::connection::{Connection, Participant};
use crate::game::{Game, Move};
use crate::messages::{move_to_command, translate_pos_back};
use crate::SERVER;

use std::fmt;
use std::io::Result as IOResult;
use std::io::Write;
use std::io::{stdin, stdout};
use std::result::Result;

// TODO add colours with https://docs.rs/ansi_term/0.12.1/ansi_term/

pub fn run() {
    let online = choice("online", "local");
    let manual = choice("manual", "bot");

    if online {
        // connect to server
        let mut conn = Connection::new(SERVER).unwrap();
        let create_new = choice("create", "join");

        // get the token, colour, and match_id
        let p: Participant;
        let match_id: String;
        if create_new {
            // create a match
            let tup = conn.create_match();
            match_id = tup.0;
            p = tup.1;
            println!("match id: {}", match_id);
        } else {
            // join a match
            match_id = input(&"match id:\n> ").unwrap().trim().to_string();
            p = conn.join_match(&match_id); // TODO ask again for match id if error
        }
        let mut game = Game::from_state_msg(conn.recv_state());
        let (red, blue) = game.get_red_blue();
        println!(
            "This game's cards:\n{}{}{}{}{}",
            red.cards[0], red.cards[1], blue.cards[0], blue.cards[1], game.table_card
        );
        println!("You are playing as {}.", if p.red { "RED" } else { "BLUE" });
        while game.in_progress {
            println!("{}", game);
            if let Color::Red = game.color {
                let my_move = if manual {
                    get_move_input(&game)
                } else {
                    get_move(&game)
                };
                conn.send(&move_to_command(&my_move, &match_id, &p.token, &game)).unwrap();
                // println!("{:#?}", conn.recv());
            }
            game = Game::from_state_msg(conn.recv_state());
        }
        println!("{}", game);

    // local
    } else {
        let random = choice("random", "preset");
        // create game
        let mut game: Game;
        if random {
            game = Game::new();
        } else {
            // ask for cards
            loop {
                let cards_result = input(&"input cards (format: [c1] [c2] [c3] [c4] [c5])\n> ")
                    .unwrap()
                    .to_lowercase()
                    .trim()
                    .split_whitespace()
                    .map(|text| Card::from_text(text))
                    .collect::<Result<Vec<Card>, _>>();
                if cards_result.is_err() {
                    println!("error parsing cards {:?}", cards_result);
                    continue;
                }
                let cards = cards_result.unwrap();
                if cards.len() != 5 {
                    println!("expected 5 cards, got {}", cards.len());
                    continue;
                }
                game = Game::from_cards(cards);
                break;
            }
        }
        // let red = choice("red", "blue")?;
        let (red, blue) = game.get_red_blue();
        println!(
            "This game's cards:\n{}{}{}{}{}",
            red.cards[0], red.cards[1], blue.cards[0], blue.cards[1], game.table_card
        );
        while game.in_progress {
            println!("{}", game);
            let my_move = if manual {
                get_move_input(&game)
            } else {
                get_move_hytak(&game)
            };
            game = game.take_turn(&my_move);
        }
        println!("{}", game);
    }
}

fn get_move_input(game: &Game) -> Move {
    loop {
        let ans = input(&"enter move (format: [card] [from] [to]) or let bot play with [bot]\n> ")
            .unwrap()
            .to_lowercase();
        // let bot play
        if ans.contains("bot") {
            return get_move_hytak(&game);
        }
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
        if !game.my.cards.iter().any(|&c| c == card) {
            println!("{} is not in your hand", card);
            continue;
        }
        // check if valid move
        let the_move = Move {
            from,
            to,
            used_left_card: game.my.cards[0] == card,
        };
        let moves = game.gen_moves();
        if moves.contains(&the_move) {
            return the_move;
        } else {
            println!("that is not a valid move");
        }
    }
}

fn choice(option_a: &str, option_b: &str) -> bool {
    let question = format!("[ {} | {} ]\n> ", option_a, option_b);
    loop {
        let ans = input(&question).unwrap().to_lowercase();
        if ans.contains(option_a) {
            return true;
        } else if ans.contains(option_b) {
            return false;
        } else {
            println!("⚠️  that's not one of the options");
        }
    }
}

fn input(message: &'_ impl fmt::Display) -> IOResult<String> {
    print!("{}", message);
    stdout().flush()?;
    let mut ret = String::new();
    stdin().read_line(&mut ret)?;
    Ok(ret)
}
