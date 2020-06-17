mod game;
mod tree_search;

use game::{Game, Status};
use tree_search::tree_search;

fn main() -> Result<(), &'static str> {
    let mut game = Game::new(4)?;
    let mut move_num = 0;

    println!("### Game started! ###");
    while let Status::Running = game.status {
        println!("{}", game);
        println!("move number: {}", move_num);
        let moves = game.possible_moves();
        println!("possible moves: {:?}", moves);
        let m = tree_search(&game, 12);
        println!("chosing move: {}", m);
        game.make_move(m)?;
        move_num += 1;
    }

    println!("###  Game ended!  ###");
    println!("{}", game);

    Ok(())
}
