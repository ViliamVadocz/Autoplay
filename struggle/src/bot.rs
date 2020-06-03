use crate::game::Game;
use std::io::Result;

pub trait StruggleBot {
    fn get_name(&self) -> &str;
    fn set_index(&mut self, index: usize);
    fn generate_move(&mut self, game: &Game) -> Result<String>;
}
