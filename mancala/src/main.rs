extern crate ordered_float;

mod game;
mod tree_search;

use game::{Game, Status, Player};
use tree_search::tree_search;

const DEBUG_PRINT: bool = false;

fn main() -> Result<(), &'static str> {
    let mut game = Game::new(4)?;
    let mut turn_num = 0;

    while let Status::Running = game.status {
        println!("{}", game);
        let moves = game.possible_moves();
        let m = match game.current_player {
            Player::First => tree_search(&game, 4)?,
            Player::Second => tree_search(&game, 8)?,
        };
        game.make_move(m)?;

        turn_num += 1;
        if DEBUG_PRINT {
            println!("turn number: {}", turn_num);
            println!("possible moves: {:?}", moves);
            println!("chosen move: {}", m);
        }
    }

    println!("{}", game);

    Ok(())
}
