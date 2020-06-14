mod game;

use game::{Game, Status};

fn main() -> Result<(), &'static str> {
    let mut game = Game::new(4);

    println!("### Game started! ###");
    
    while let Status::Running = game.status {
        println!("{}", game);
        let moves = game.possible_moves();
        println!("possible moves: {:?}", moves);
        let m = *moves.last().unwrap();
        println!("chosing move: {}", m);
        game.make_move(m)?;
    }

    println!("###  Game ended!  ###");
    println!("{}", game);

    Ok(())
}
