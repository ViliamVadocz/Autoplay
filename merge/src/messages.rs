use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "message_type")]
pub enum Message {
    #[serde(rename = "start")]
    Start(StartMsg),
    #[serde(rename = "move")]
    Move(MoveMsg),
    #[serde(rename = "error")]
    Error(ErrorMsg),
    #[serde(rename = "end")]
    End(EndMsg),
}

#[derive(Debug, Deserialize)]
pub struct StartMsg {
    pub message: String,
    pub your_side: i64,
    pub your_turn: bool,
    pub full_board: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveMsg {
    pub message: String,
    pub move_played: String,
    pub your_turn: bool,
    pub full_board: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorMsg {
    pub message: String,
    pub command_received: String,
}

#[derive(Debug, Deserialize)]
pub struct EndMsg {
    pub message: String,
    pub move_played: String,
    pub winner: i64,
    pub full_board: String,
}
