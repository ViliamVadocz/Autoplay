extern crate ordered_float;
extern crate rand;
extern crate rand_distr;
extern crate rulinalg;

mod agent;
mod game;
mod network;

use agent::{Agent, SimpleAgent};
use game::{Game, Player, Status};
use network::{rect_lin_unit, Network};

const DEBUG_PRINT: bool = false;

fn main() -> Result<(), &'static str> {
    let mut game = Game::new(4)?;
    let mut turn_num = 0;

    let a = SimpleAgent {};
    let n = Network::from(&[112, 56, 28, 1], rect_lin_unit)?;
    // n.save("test_b.data", "test_w.data")?;
    // let nn = Network::from_save("test_b.data", "test_w.data", rect_lin_unit)?;

    while let Status::Running = game.status {
        println!("{}", game);
        let moves = game.possible_moves();
        let m = match game.current_player {
            Player::First => a.tree_search(&game, 10)?,
            Player::Second => n.tree_search(&game, 5)?,
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
