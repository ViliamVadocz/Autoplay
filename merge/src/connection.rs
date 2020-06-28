use std::{
    io::{prelude::*, BufReader, BufWriter, Result, Error, ErrorKind},
    net::TcpStream,
};

use crate::{
    bot::Bot,
    messages::Message,
    game::Game,
};

pub fn run_bot<B: Bot>(mut bot: B, address: &str) -> Result<()> {
    // get connected
    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    println!("connected!");

    // get first message
    if let Message::Start(start_msg) = recv(&mut reader)? {
        bot.set_side(start_msg.your_side);
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "did not receive a start message",
        ))
    }

    let mut game = Game::new();
    loop {
        match recv(&mut reader)? {
            Message::Start(_) => return Err(Error::new(
                ErrorKind::InvalidData,
                "received a second start message",
            )),
            Message::Error(err_msg) => println!("{:?}", err_msg),
            Message::End(end_msg) => return Ok(()),
            Message::Move(move_msg) => {
                // game.update(move_msg);
                send(&mut writer, &"0 1".to_string())?;
            }
        };
    }
}

fn recv<R: BufRead>(reader: &mut R) -> Result<Message> {
    let mut msg = String::new();
    reader.read_line(&mut msg)?;
    let message = serde_json::from_str::<Message>(&msg)?;
    println!("received message:\n{:?}", message);
    Ok(message)
}

fn send<W: Write>(writer: &mut W, msg: &String) -> Result<()> {
    // you could do some msg processing such as making sure you end message with a newline
    writer.write_all(msg.as_bytes())
}
