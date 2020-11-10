use crate::error::{Result, UnexpectedMessage};
use crate::messages::*;
use std::marker::Send;
use websocket::result::WebSocketResult;
use websocket::sync::{client::Client, stream::NetworkStream};
use websocket::{ClientBuilder, Message, OwnedMessage};

pub struct Participant {
    pub token: String,
    pub red: bool,
}

pub struct Connection {
    client: Client<Box<dyn NetworkStream + Send>>,
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
            OwnedMessage::Close(_) => {
                panic!("WebSocket closed when trying to receive LitamaMessage")
            }
            OwnedMessage::Ping(_) => panic!("Received ping when trying to receive LitamaMessage"),
            OwnedMessage::Pong(_) => panic!("Received pong when trying to receive LitamaMessage"),
        };
        Ok(message)
    }

    pub fn create_match(&mut self) -> Result<(String, Participant)> {
        self.send("create")?;
        let message = self.recv()?;
        if let LitamaMessage::Create(msg) = message {
            Ok((
                msg.match_id,
                Participant {
                    token: msg.token,
                    red: color_is_red(msg.color)?,
                },
            ))
        } else {
            Err(Box::new(UnexpectedMessage::new(message)))
        }
    }

    pub fn join_match(&mut self, match_id: &str) -> Result<Participant> {
        self.send(&format!("join {}", match_id))?;
        let message = self.recv()?;
        if let LitamaMessage::Join(msg) = message {
            Ok(Participant {
                token: msg.token,
                red: color_is_red(msg.color)?,
            })
        } else {
            Err(Box::new(UnexpectedMessage::new(message)))
        }
    }

    pub fn recv_state(&mut self) -> Result<StateMsg> {
        let message = self.recv()?;
        match message {
            LitamaMessage::State(message) => Ok(message),
            _ => Err(Box::new(UnexpectedMessage::new(message))),
        }
    }
}
