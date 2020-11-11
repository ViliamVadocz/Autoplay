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
    pub fn new(address: &str) -> WebSocketResult<Connection> {
        let client = ClientBuilder::new(address)
            .expect("Couldn't parse server address")
            .connect(None)?;
        Ok(Connection { client })
    }

    pub fn send(&mut self, text: &str) -> WebSocketResult<()> {
        self.client.send_message(&Message::text(text))
    }

    pub fn recv(&mut self) -> WebSocketResult<LitamaMessage> {
        let message = match self.client.recv_message()? {
            // if it's a parse error then the server protocol has probably changed and retrying won't help
            OwnedMessage::Text(text) => serde_json::from_str::<LitamaMessage>(&text).unwrap(),
            OwnedMessage::Binary(bytes) => serde_json::from_slice::<LitamaMessage>(&bytes).unwrap(),
            // another message that is not handled
            // OwnedMessage::Ping(_) => panic!("todo"),
            // OwnedMessage::Pong(_) => panic!("todo"),
            // OwnedMessage::Close(_) => panic!("todo"),
            msg => panic!("Received unexpected message: {:#?}", msg),
        };
        Ok(message)
    }

    pub fn create_match(&mut self) -> (String, Participant) {
        // send create message
        while let Err(err) = self.send("create") {
            println!("Error while sending: {}", err);
        }
        // try to receive response
        loop {
            match self.recv() {
                Ok(LitamaMessage::Create(msg)) => {
                    return (
                        msg.match_id,
                        Participant {
                            token: msg.token,
                            red: color_is_red(msg.color).unwrap(),
                        },
                    )
                }
                Ok(LitamaMessage::Error(msg)) => println!("Received error message: {}", msg.error),
                Ok(msg) => println!("Received wrong message type {:#?}", msg),
                Err(err) => println!("Error while receiving: {}", err),
            }
        }
    }

    pub fn join_match(&mut self, match_id: &str) -> Participant {
        // send join message
        let to_send_msg = format!("join {}", match_id);
        while let Err(err) = self.send(&to_send_msg) {
            println!("Error while sending: {}", err)
        }
        // try to receive response
        loop {
            match self.recv() {
                Ok(LitamaMessage::Join(msg)) => {
                    return Participant {
                        token: msg.token,
                        red: color_is_red(msg.color).unwrap(),
                    }
                }
                Ok(LitamaMessage::Error(msg)) => println!("Received error message: {}", msg.error),
                Ok(msg) => println!("Received wrong message type {:#?}", msg),
                Err(err) => println!("Error while receiving: {}", err),
            }
        }
    }

    pub fn recv_state(&mut self) -> StateMsg {
        loop {
            match self.recv() {
                Ok(LitamaMessage::State(msg)) => return msg,
                Ok(msg) => println!("Received wrong message type {:#?}", msg),
                Err(err) => println!("Error while receiving: {}", err),
            }
        }
    }
}
