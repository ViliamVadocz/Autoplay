use crate::game::Game;
use crate::cards::Card;
use std::env;
use std::result::Result;



pub type Args = (Playing, GameHost);

pub enum Playing {
    Human,
    Bot,
    No,
}

pub type MatchId = String;
pub type Username = String;

pub enum GameHost {
    Local(Game),
    Online(Option<MatchId>, Username),
}

pub fn parse_args() -> Result<Args, String> {
    let mut args = env::args().map(|s| s.to_lowercase());
    // ignore first argument
    let _exe = args.next().ok_or_else(|| "How did you launch this without any arguments?".to_string())?;

    let online = match args.next().as_deref() {
        Some("online") => Ok(true),
        Some("local") => Ok(false),
        Some("help") => Err("".to_string()),
        Some(word) => Err(format!("Unrecognised word: {}", word)),
        None => Err("No command found".to_string()),
    }?;

    let second = args.next();
    if online {
        // find out what to do online
        let (need_match_id, spectate) = match second.as_deref() {
            Some("join") => Ok((true, false)),
            Some("spectate") => Ok((true, true)),
            Some("create") => Ok((false, false)),
            Some(word) => Err(format!("Unrecognised word: {}", word)),
            None => Err("You must specify whether to create or join an online match".to_string()),
        }?;
        // potentially get match id
        let match_id = if need_match_id {
            let a = args.next();
            if a.is_none() { return Err("You need give a match id".to_string()); }
            a
        } else {
            None
        };
        let username = match args.next() {
            Some(name) => Ok(name),
            None => Err("You must enter a username when playing online".to_string()),
        }?;
        // find out if human is playing
        let human = is_human(args.next())?;
        let playing = if human {
            Playing::Human
        } else if spectate {
            Playing::No
        } else {
            Playing::Bot
        };

        Ok((playing, GameHost::Online(match_id, username)))
    
    } else {
        // find out if I want to use preset cards or random
        let preset = match second.as_deref() {
            Some("preset") => Ok(true),
            Some("random") => Ok(false),
            Some(word) => Err(format!("Unrecognised word: {}", word)),
            None => Err("You must specify whether to use random or preset cards".to_string()),
        }?;
        // make a game struct for use later
        let game = if preset {
            let mut cards = Vec::new();
            for i in 0..5 {
                cards.push(Card::from_text(&args.next().ok_or_else(|| format!("Expected 5 cards, got {}", i))?)?)
            }
            Game::from_cards(cards)
        } else {
            // random cards
            Game::new()
        };
        // find out if human is playing
        let playing = if is_human(args.next())? {Playing::Human} else {Playing::Bot};

        Ok((playing, GameHost::Local(game)))
    }
}

fn is_human(hb: Option<String>) -> Result<bool, String> {
    match hb.as_deref() {
        Some("-h") => Ok(true),
        Some(word) => Err(format!("Unrecognised word: {}", word)),
        None => Ok(false)
    }
}
