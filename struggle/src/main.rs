mod messages;
mod bot;
mod game;
mod card;

use std::io::Result;
use crate::bot::StruggleBot;

#[macro_use]
extern crate serde_derive;


fn main() -> Result<()> {
    let mut my_bot = StruggleBot::new("Will");
    my_bot.connect("kris6673.synology.me:55445")?;
    // my_bot.connect("struggle.iameld3st.xyz:55445")?;
    my_bot.run(true)
}
