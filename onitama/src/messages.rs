use crate::game::{Game, Move};
use std::result::Result;

#[derive(Debug, Deserialize)]
#[serde(tag = "messageType")]
pub enum LitamaMessage {
    #[serde(rename = "create")]
    Create(CreateMsg),
    #[serde(rename = "join")]
    Join(JoinMsg),
    #[serde(rename = "state")]
    State(Box<StateMsg>),
    #[serde(rename = "move")]
    Move(MoveMsg),
    #[serde(rename = "spectate")]
    Spectate(SpectateMsg),
    #[serde(rename = "error")]
    Error(ErrorMsg),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMsg {
    pub match_id: String,
    pub token: String,
    pub index: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinMsg {
    pub match_id: String,
    pub token: String,
    pub index: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMsg {
    pub usernames: UsernamesObj,
    pub indices: IndicesObj,
    pub match_id: String,
    pub current_turn: String,
    pub cards: CardsObj,
    // pub starting_cards: CardsObj,
    // pub moves: Vec<String>,
    pub board: String,
    pub game_state: String,
    pub winner: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernamesObj {
    pub red: String,
    pub blue: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndicesObj {
    pub red: u8,
    pub blue: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardsObj {
    pub red: [String; 2],
    pub blue: [String; 2],
    pub side: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveMsg {
    pub match_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpectateMsg {
    pub match_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMsg {
    pub match_id: String,
    pub error: String,
    pub command: String,
}

pub fn is_in_progress(game_state: String) -> Result<bool, String> {
    match game_state.as_ref() {
        "waiting for player" => Ok(true),
        "in progress" => Ok(true),
        "ended" => Ok(false),
        _ => Err(format!("Unknown game state: {}", game_state)),
    }
}

pub fn translate_pos(pos: usize) -> String {
    let row = pos / 5;
    let col = pos % 5;
    [
        "edcba".chars().nth(col).unwrap(),
        "12345".chars().nth(row).unwrap(),
    ]
    .iter()
    .collect::<String>()
}

pub fn parse_pos(pos: &str) -> Result<usize, String> {
    let mut chars = pos.chars();
    let first = chars.next().ok_or("Given pos is too short")?;
    let second = chars.next().ok_or("Given pos is too short")?;
    let col = "edcba"
        .find(first)
        .ok_or(format!("`{}` is an invalid column", first))?;
    let row = "12345"
        .find(second)
        .ok_or(format!("`{}` is an invalid row", second))?;
    Ok(row * 5 + col)
}

pub fn move_to_command(my_move: &Move, match_id: &str, token: &str, game: &Game) -> String {
    let pos = format!(
        "{}{}",
        translate_pos(my_move.from as usize),
        translate_pos(my_move.to as usize)
    );
    let card = if my_move.used_left_card {
        &game.my.cards[0]
    } else {
        &game.my.cards[1]
    };
    format!("move {} {} {} {}", match_id, token, pos, card.get_name())
}
