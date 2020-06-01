#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "msgType")]
pub enum Message {
    #[serde(rename = "error")]
    GameError(ErrorMessage),
    #[serde(rename = "setup")]
    Setup(SetupMessage),
    #[serde(rename = "game")]
    GameInfo(GameMessage),
    #[serde(rename = "gameOver")]
    GameOver(GameOverMessage),
}

// error
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error_code: i32,
}

pub fn get_error_name(code: i32) -> &'static str {
    match code {
        -1 => "UnknownError",

        100 => "NotYourTurn",
        101 => "DeckIsEmpty",
        102 => "NotInCenter",
        103 => "NotInHand",
        104 => "NotValidTrick",
        105 => "HandIsFull",
        
        200 => "GameIsNotRunning",
        201 => "YouAreNotInGame",
        
        300 => "UnexpectedMessage",
        301 => "InvalidMessage",
        302 => "UnknownAction",
        303 => "NoPlayerName",
        _ => "???"
    }
}

// setup
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupMessage {
    pub index: usize,
    pub players: Vec<String>,
}


// game
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMessage {
    pub hand: Vec<String>,
    pub game_state: GameState,
}

// game over
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOverMessage {
    pub winner: usize,
    pub game_state: GameState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub current_player: usize,
    pub has_moves: bool,
    pub players: Vec<Player>,
    pub deck_size: u32,
    pub center: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub hand_size: u32,
    pub tricks_played: Vec<Vec<String>>,
    pub last_action: Option<LastAction>,
    pub score: u32,
    pub has_left: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastAction {
    pub action_type: String,
    pub card: Option<String>,
    #[serde(default)]
    pub cards: Vec<String>,
}
