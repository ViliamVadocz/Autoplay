use std::{
    io::{BufReader, Error, ErrorKind, Result, prelude::*},
    net::TcpStream
};

use crate::{
    messages::{
        get_error_name,
        SetupMessage,
        Message::{self, *}
    },
    game::Game,
    bot::StruggleBot
};

pub fn run_bot<B: StruggleBot>(mut bot: B, address: &str) -> Result<()> {
    let (mut stream, setup_msg) = connect(bot.get_name(), address)?;
    let bot_index = setup_msg.index;
    bot.set_index(bot_index);
    let mut game = Game::new(setup_msg.players);
    

    loop {
        // receive response
        match receive(&stream)? {
            GameError(message) => {
                let error_text = get_error_name(message.error_code);
                println!("error: {}", error_text);

                // check if server expects a response after error
                if [101, 102, 103, 104, 105, 300, 301, 302].contains(&message.error_code) {
                    let action = bot.generate_move(&game)?;
                    send(&mut stream, action)?;
                }
            },

            Setup(_) => return Err(Error::new(ErrorKind::InvalidData, "received unexpected \"setup\" message")),

            GameOver(message) => {
                let winner = message.winner;
                let winner_name = &message.game_state.players[winner].name;
                println!("game over! winner: {}", winner_name);
                for player in message.game_state.players.iter() {
                    println!("{0}: {1}", player.name, player.score);
                }
                return Ok(())
            },

            GameInfo(message) => {
                game.update(message);

                // ask bot for action
                if game.has_moves && game.current_player_index == bot_index {
                    println!("hand: {:?}", &game.my_hand);
                    let action = bot.generate_move(&game)?;
                    send(&mut stream, action)?;
                }
            }
        };
    }
}

fn connect(name: &str, address: &str) -> Result<(TcpStream, SetupMessage)> {
    // get connected
    let mut stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;
    println!("connected!");

    // set name
    stream.write(format!("join \"{}\"\n", name).as_bytes())?;

    // wait for setup message once a game starts
    let response = receive(&stream)?;
    if let Setup(message) = response {
        println!("received setup, players: {}", message.players.len());
        Ok((stream, message))
    } else {
        // invalid response type
        println!("{:?}", response);
        Err(Error::new(ErrorKind::InvalidData, "the type received was not \"setup\""))
    }
}

fn receive(stream: &TcpStream) -> Result<Message> {
    let mut msg = String::new();
    BufReader::new(stream).read_line(&mut msg)?;
    // println!("{}", msg.as_str());
    Ok(serde_json::from_str::<Message>(msg.as_str())?)
}

fn send(mut stream: &TcpStream, mut msg: String) -> Result<usize> {
    remove_newlines(&mut msg);
    msg.push_str(&"\n");
    stream.write(msg.as_bytes())
}

fn remove_newlines(s: &mut String) {
    s.retain(|c| c != '\n');
}
