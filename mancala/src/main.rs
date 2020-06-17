mod game;
mod tree_search;

use game::{Game, Status};
use tree_search::tree_search;

fn main() -> Result<(), &'static str> {
    let mut game = Game::new(4)?;
    let mut turn_num = 0;

    println!("### Game started! ###");
    while let Status::Running = game.status {
        println!("{}", game);
        println!("turn number: {}", turn_num);
        // let moves = game.possible_moves();
        // println!("possible moves: {:?}", moves);
        let m = tree_search(&game, 10)?;
        println!("chosen move: {}", m);
        game.make_move(m)?;
        turn_num += 1;
    }

    println!("###  Game ended!  ###");
    println!("{}", game);

    Ok(())
}
