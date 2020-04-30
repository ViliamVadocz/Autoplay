use std::{
    option::Option,
    io::{self, BufReader, Result, prelude::*}
};
use std::net::TcpStream;

use crate::{
    messages::{
        get_error_name,
        Message::{self, *}
    },
    game::Game
};

pub struct StruggleBot {
    name: &'static str,
    index: usize,
    stream: Option<TcpStream>,
    game: Game
}

impl StruggleBot {
    pub fn new(name: &'static str) -> StruggleBot {
        StruggleBot {
            name,
            index: 0,
            stream: None,
            game: Game::new()
        }
    }

    pub fn connect(&mut self,  address: &str) -> Result<()> {
        // get connected
        let mut stream = TcpStream::connect(address)?;
        stream.set_nodelay(true)?;
        println!("connected!");

        // set name
        stream.write(format!("join \"{}\"\n", self.name).as_bytes())?;
        self.stream = Some(stream);
        // wait for setup message
        let response = self.receive()?;
        if let Setup(message) = response {
            println!("received setup, players: {}", message.players.len());
            // update self
            self.game.start(message.players);
            self.index = message.index;
            Ok(())
        } else {
            println!("{:?}", response);
            panic!("the type received was not \"setup\"")
        }
    }

    fn receive(&self) -> Result<Message> {
        let mut msg = String::new();
        BufReader::new(self.stream.as_ref().unwrap()).read_line(&mut msg)?;
        // println!("{}", msg.as_str());
        Ok(serde_json::from_str::<Message>(msg.as_str())?)
    }

    pub fn run(mut self, manual: bool) -> Result<()> {
        loop {
            // receive response
            match self.receive()? {
                GameError(message) => {
                    let error_text = get_error_name(message.error_code);
                    println!("error: {}", error_text);

                    // check if server expects a response after error
                    if [101, 102, 103, 104, 105, 300, 301, 302].contains(&message.error_code) {
                        self.take_action(manual)?;
                    }
                },

                Setup(_message) => {
                    panic!("received unexpected \"setup\" message");
                },

                GameOver(message) => {
                    let winner = message.winner;
                    let winner_name = &message.game_state.players[winner].name;
                    println!("game over! winner: {}", winner_name);
                    for player in message.game_state.players.iter() {
                        println!("{0}: {1}", player.name, player.score);
                    }
                    return Ok(());
                },

                GameInfo(message) => {
                    // TODO update Game object and use it to print state
                    self.game.update(message);
                    // my turn
                    if self.game.has_moves && self.game.current_player_index == self.index {
                        println!("hand: {:?}", self.game.my_hand);
                        self.take_action(manual)?;
                    }
                }
            };
        }
    }

    fn take_action(&self, manual: bool) -> Result<()> {
        let mut action: String;
        if manual {
            action = get_user_input()?;
            remove_newlines(&mut action);
        } else {
            // TODO determine move
            action = String::from("draw");
        }
        action.push_str(&"\n");
        self.stream.as_ref().unwrap().write(action.as_bytes())?;
        Ok(())
    }
}

fn get_user_input() -> Result<String> {
    println!("your action: ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!("");
    Ok(buffer)
}

fn remove_newlines(s: &mut String) {
    s.retain(|c| c != '\n');
}
