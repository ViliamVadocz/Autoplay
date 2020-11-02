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
    pub red: Vec<String>,
    pub blue: Vec<String>,
    pub side: String
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
