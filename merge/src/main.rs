extern crate bitmaps;
extern crate typenum;
extern crate serde;
extern crate serde_json;

mod messages;
mod connection;
mod game;
mod bot;

use std::io::Result;

fn main() -> Result<()> {
    Ok(())

    // let white = [
    //     0, 1, 2, 3,
    //     8, 9, 10,
    //     16, 17,
    //     24
    // ];
    // let red = [
    //                 39,
    //             46, 47,
    //         53, 54, 55,
    //     60, 61, 62, 63
    // ];
    
    // let mut w: Bitmap<U64> = Bitmap::new();
    // let mut r: Bitmap<U64> = Bitmap::new();
    // for pos in 0..64 {
    //     if white.contains(&pos) {
    //         w.set(pos, true);
    //     } else if red.contains(&pos) {
    //         r.set(pos, true);
    //     }
    // }
    // println!("white\n{}", bitmap_to_string(w));
    // println!("red\n{}", bitmap_to_string(r));
    // println!("w: {}, r: {}", w.into_value(), r.into_value());
}
