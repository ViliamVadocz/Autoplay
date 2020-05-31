use std::io::Result;
use crate::game::Game;

pub trait StruggleBot {
    fn get_name(&self) -> &str;
    fn set_index(&mut self, index: usize);
    fn generate_move(&mut self, game: &Game) -> Result<String>;
}
