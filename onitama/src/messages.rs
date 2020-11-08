use crate::error::{Result, UnexpectedVariant};
use crate::game::{Game, Move};

#[derive(Debug, Deserialize)]
#[serde(tag = "messageType")]
pub enum LitamaMessage {
    #[serde(rename = "create")]
    Create(CreateMsg),
    #[serde(rename = "join")]
    Join(JoinMsg),
    #[serde(rename = "state")]
    State(StateMsg),
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
    pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinMsg {
    pub match_id: String,
    pub token: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMsg {
    pub match_id: String,
    pub current_turn: String,
    pub cards: CardsObj,
    // pub starting_cards: CardsObj,
    pub moves: Vec<String>,
    pub board: String,
    pub game_state: String,
    pub winner: String,
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

pub fn color_is_white(color: String) -> Result<bool> {
    match color.as_ref() {
        "red" => Ok(true),
        "blue" => Ok(false),
        _ => Err(Box::new(UnexpectedVariant::new(color))),
    }
}

pub fn is_in_progress(game_state: String) -> Result<bool> {
    match game_state.as_ref() {
        "waiting for player" => Ok(true),
        "in progress" => Ok(true),
        "ended" => Ok(false),
        _ => Err(Box::new(UnexpectedVariant::new(game_state))),
    }
}

pub fn translate_pos(pos: usize) -> String {
    let row = pos / 5;
    let col = pos % 5;
    [
        "abcde".chars().nth(row).unwrap(),
        "12345".chars().nth(col).unwrap(),
    ]
    .iter()
    .collect::<String>()
}

pub fn translate_pos_back(pos: &str) -> Result<usize> {
    let mut chars = pos.chars();
    let first = chars.next().ok_or("pos too short")?;
    let second = chars.next().ok_or("pos too short")?;
    let row = "abcde"
        .find(first)
        .ok_or(format!("{} is an invalid row", first))?;
    let col = "12345"
        .find(second)
        .ok_or(format!("{} is an invalid col", second))?;
    Ok(row * 5 + col)
}

pub fn move_to_command(my_move: &Move, match_id: &str, token: &str, game: &Game) -> String {
    let mut command = String::from("move ");
    // match id
    command.push_str(match_id);
    command.push(' ');
    // token
    command.push_str(token);
    command.push(' ');
    // from:to
    command.push_str(&translate_pos(my_move.from as usize));
    command.push_str(&translate_pos(my_move.to as usize));
    command.push(' ');
    // card
    let my_cards = if game.white_to_move {
        &game.white.cards
    } else {
        &game.black.cards
    };
    let card = if my_move.used_left_card {
        &my_cards[0]
    } else {
        &my_cards[1]
    };
    command.push_str(card.get_name());
    command
}
