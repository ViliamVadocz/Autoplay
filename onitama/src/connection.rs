use crate::messages::*;
use crate::error::{Result, UnexpectedMessage, UnexpectedVariant};
use websocket::sync::{client::Client, stream::NetworkStream};
use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::result::WebSocketResult;
use std::marker::Send;

pub struct Player {
    pub token: String,
    pub white: bool,
}

pub struct Connection {
    client: Client<Box<dyn NetworkStream + Send>>
}

impl Connection {
    pub fn new(address: &str) -> Result<Connection> {
        let client = ClientBuilder::new(address)?.connect(None)?;
        Ok(Connection { client })
    }

    pub fn send(&mut self, text: &str) -> WebSocketResult<()> {
        self.client.send_message(&Message::text(text))
    }

    pub fn recv(&mut self) -> Result<LitamaMessage> {
        let message = match self.client.recv_message()? {
            OwnedMessage::Text(text) => serde_json::from_str::<LitamaMessage>(&text)?,
            OwnedMessage::Binary(bytes) => serde_json::from_slice::<LitamaMessage>(&bytes)?,
            OwnedMessage::Close(_) => panic!("WebSocket closed when trying to receive LitamaMessage"),
            OwnedMessage::Ping(_) => panic!("Received ping when trying to receive LitamaMessage"),
            OwnedMessage::Pong(_) => panic!("Received pong when trying to receive LitamaMessage"),
        };
        Ok(message)
    }

    pub fn create_match(&mut self) -> Result<(String, Player)> {
        self.send("create")?;
        let message = self.recv()?;
        if let LitamaMessage::Create(msg) = message {
            Ok((msg.match_id, Player { token: msg.token, white: color_is_white(msg.color)?} ))
        } else {
            Err(Box::new(UnexpectedMessage::new(message)))
        }
    }

    pub fn join_match(&mut self, match_id: &str) -> Result<Player> {
        self.send(&format!("join {}", match_id))?;
        let message = self.recv()?;
        if let LitamaMessage::Join(msg) = message {
            Ok(Player { token: msg.token, white: color_is_white(msg.color)? })
        } else {
            Err(Box::new(UnexpectedMessage::new(message)))
        }
    }

    pub fn recv_state(&mut self) -> Result<StateMsg> {
        let message = self.recv()?;
        match message {
            LitamaMessage::State(message) => Ok(message),
            _ => Err(Box::new(UnexpectedMessage::new(message)))
        }
    }
}
