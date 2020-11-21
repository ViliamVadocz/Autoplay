use crate::game::{Game, Move};
use crate::messages::*;
use std::marker::Send;
use std::result::Result;
use websocket::result::WebSocketResult;
use websocket::sync::{client::Client, stream::NetworkStream};
use websocket::{ClientBuilder, Message, OwnedMessage};

macro_rules! recv_loop {
    ($self:ident, $p:pat => $e:expr) => {
        loop {
            match $self.recv() {
                $p => break $e,
                Ok(LitamaMessage::Error(msg)) => println!("Received error message: {}", msg.error),
                Ok(msg) => println!("Received wrong message type {:#?}", msg),
                Err(err) => println!("Error while receiving: {}", err),
            }
        }
    };
}

pub struct Participant {
    pub token: String,
    pub index: u8,
}

pub struct Connection {
    client: Client<Box<dyn NetworkStream + Send>>,
}

impl Connection {
    pub fn new(address: &str) -> Result<Connection, String> {
        let client = ClientBuilder::new(address)
            .map_err(|e| e.to_string())?
            .connect(None)
            .map_err(|e| e.to_string())?;
        Ok(Connection { client })
    }

    fn send(&mut self, text: &str) -> WebSocketResult<()> {
        // println!("-> {}", text);
        self.client.send_message(&Message::text(text))
    }

    fn recv(&mut self) -> WebSocketResult<LitamaMessage> {
        let message = loop {
            match self.client.recv_message()? {
                // if it's a parse error then the server protocol has probably changed and retrying won't help
                OwnedMessage::Text(text) => {
                    // println!("<- {}", text);
                    break serde_json::from_str::<LitamaMessage>(&text).unwrap();
                }
                OwnedMessage::Binary(bytes) => {
                    break serde_json::from_slice::<LitamaMessage>(&bytes).unwrap()
                }
                // another message that is not handled
                OwnedMessage::Ping(data) => {
                    self.client.send_message(&OwnedMessage::Pong(data)).unwrap()
                }
                // OwnedMessage::Pong(_) => panic!("todo"),
                // OwnedMessage::Close(_) => panic!("todo"),
                msg => panic!("Received unexpected message: {:#?}", msg),
            }
        };
        Ok(message)
    }

    pub fn create_match(&mut self, username: &str) -> (String, Participant) {
        // send create message
        while let Err(err) = self.send(&format!("create {}", username)) {
            println!("Error while sending: {}", err);
        }
        return recv_loop!(
            self,
            Ok(LitamaMessage::Create(msg)) => (
                msg.match_id,
                Participant {
                    token: msg.token,
                    index: msg.index,
                },
            )
        );
    }

    pub fn join_match(&mut self, match_id: &str, username: &str) -> Participant {
        // send join message
        while let Err(err) = self.send(&format!("join {} {}", match_id, username)) {
            println!("Error while sending: {}", err)
        }
        return recv_loop!(
            self,
            Ok(LitamaMessage::Join(msg)) => Participant {
                token: msg.token,
                index: msg.index,
            }
        );
    }

    pub fn spectate(&mut self, match_id: &str) -> StateMsg {
        // send spectate message
        while let Err(err) = self.send(&format!("spectate {}", match_id)) {
            println!("Error while sending: {}", err)
        }
        // confirm spectate
        recv_loop!(self, Ok(LitamaMessage::Spectate(_)) => ());
        loop {
            let state_msg = self.recv_state();
            if state_msg.game_state != "waiting for player" {
                break state_msg;
            }
        }
    }

    pub fn recv_state(&mut self) -> StateMsg {
        return recv_loop!(self, Ok(LitamaMessage::State(msg)) => *msg);
    }

    pub fn make_move(&mut self, m: &Move, match_id: &str, token: &str, game: &Game) -> StateMsg {
        // send move
        while let Err(err) = self.send(&move_to_command(m, match_id, token, game)) {
            println!("Error while sending: {}", err)
        }
        // confirm move
        recv_loop!(self, Ok(LitamaMessage::Move(_)) => ());
        self.recv_state()
    }
}
