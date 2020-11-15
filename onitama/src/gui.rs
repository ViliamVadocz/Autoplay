use crate::SERVER;
use crate::connection::{Connection, Participant};
use crate::cli::{Args, Playing, GameHost};
use crate::game::{Game, Move};
use crate::bot::get_move;
use crate::color::Color;
use std::result::Result;

pub fn run(args: Args) -> Result<(), String> {
    // TODO launch a second thread with gui
    let (playing, host) = args;
    match host {
        GameHost::Local(mut game) => {
            let my_color = Color::Red; // TODO pick randomly
            while game.in_progress {
                let the_move = if my_color == game.color {
                    // my turn
                    match playing {
                        Playing::Human => todo(&game), // TODO get move from second thread
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    }
                } else {
                    // otherwise bot plays
                    get_move(&game)
                };
                game = game.take_turn(&the_move);
            }
        },
        GameHost::Online(maybe_match_id, username) => {
            let mut conn = Connection::new(SERVER)?;

            let (match_id, p) = match maybe_match_id {
                Some(match_id) => {
                    let p = if matches!(playing, Playing::No) {
                        // fake participant
                        Participant {
                            token: String::new(),
                            color: Color::Red,
                        }
                    } else {
                        conn.join_match(&match_id, &username)
                    };
                    (match_id, p)
                },
                None => conn.create_match(&username),
            };

            let mut state_msg = conn.spectate(&match_id);
            let mut game = Game::from_state_msg(state_msg);
            while game.in_progress {
                // TODO display game
                if p.color == game.color && !matches!(playing, Playing::No) {
                    let my_move = match playing {
                        Playing::Human => todo(&game), // TODO get move from second thread
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    };
                    state_msg = conn.make_move(&my_move, &match_id, &p.token, &game);
                }
                else {
                    state_msg = conn.recv_state();
                }
                game = Game::from_state_msg(state_msg);
            }
        },
    };
    Ok(())
}

// placeholder function for communicating with gui to get user input
fn todo(g: &Game) -> Move {
    g.gen_moves().pop().unwrap()
}

// WIP GUI CODE BELOW

// use sdl2::event::Event;
// use sdl2::image::LoadTexture;
// use sdl2::keyboard::Keycode;
// use sdl2::mouse::MouseButton;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::{TextureQuery, WindowCanvas};

// use std::time::Duration;

// use bitwise::TestBit;

// let sdl_context = sdl2::init()?;
// let video_subsystem = sdl_context.video()?;
// let ttf_context = sdl2::ttf::init()?;

// let window = video_subsystem
//     .window("Onitama", BOARD_SIZE + SIDEBAR, BOARD_SIZE)
//     .position_centered()
//     .opengl()
//     .build()?;

// let mut canvas = window.into_canvas().build()?;

// let texture_creator = canvas.texture_creator();

// // load piece images
// let red_pawn = texture_creator.load_texture("./images/red_pawn.png")?;
// let red_king = texture_creator.load_texture("./images/red_king.png")?;
// let blue_pawn = texture_creator.load_texture("./images/blue_pawn.png")?;
// let blue_king = texture_creator.load_texture("./images/blue_king.png")?;

// let font = ttf_context.load_font("./fonts/arial.ttf", 20)?;
// let draw_card = |canvas: &mut WindowCanvas, x, y, card: &Card, right_side_up| -> Result<()> {
//     let surface = font.render(card.get_name()).blended(Color::RED)?;
//     let texture = texture_creator.create_texture_from_surface(&surface)?;
//     let TextureQuery { width, height, .. } = texture.query();
//     if right_side_up {
//         canvas.copy(
//             &texture,
//             None,
//             Some(Rect::new(
//                 x,
//                 y,
//                 width,
//                 height,
//             )),
//         )?;
//     } else {
//         canvas.copy_ex(&texture, None, Some(Rect::new(
//             x + 100 - width as i32,
//             y + 100,
//             width,
//             height,
//         )
//         ), 0.0, None, true, true)?;
//     }

//     // display move
//     let pad = if right_side_up { height as i32 } else { 0 };
//     let pieces = if right_side_up {
//         card.get_red()
//     } else {
//         card.get_blue()
//     };
//     for i in 0..25 {
//         let row = i / 5;
//         let col = i % 5;
//         canvas.set_draw_color(if pieces.test_bit(i as u32) {
//             Color::BLACK
//         } else {
//             Color::WHITE
//         });
//         let rect = Rect::new(
//             x + col * CARD_SQUARE_SIZE as i32,
//             pad + y + row * CARD_SQUARE_SIZE as i32,
//             CARD_SQUARE_SIZE,
//             CARD_SQUARE_SIZE,
//         );
//         canvas.fill_rect(rect)?;
//         canvas.set_draw_color(Color::BLACK);
//         canvas.draw_rect(rect)?;
//     }

//     Ok(())
// };

// canvas.set_draw_color(BACKGROUND_COLOR);
// canvas.clear();
// canvas.present();

// let mut game = game::Game::new();
// draw_card(&mut canvas, 700, 50, &game.red.cards[0], false)?;
// draw_card(&mut canvas, 850, 50, &game.red.cards[1], false)?;
// draw_card(&mut canvas, 700, 458, &game.blue.cards[0], true)?;
// draw_card(&mut canvas, 850, 458, &game.blue.cards[1], true)?;
// draw_card(&mut canvas, 750, 254, &game.table_card, false)?;

// let mut event_pump = sdl_context.event_pump()?;
// let mut clicked_square = None;
// 'main_loop: loop {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => {
//                 break 'main_loop;
//             }
//             Event::MouseButtonDown {
//                 mouse_btn: MouseButton::Left,
//                 x,
//                 y,
//                 ..
//             } => {
//                 clicked_square = get_pos_from_click(x, y);
//             }
//             _ => {}
//         }
//     }

//     // draw chequerboard
//     for i in 0..25u8 {
//         let row = i / 5;
//         let col = i % 5;
//         let color = if clicked_square.is_some() && i == clicked_square.unwrap() {
//             Color::RGB(100, 200, 100)
//         } else if i % 2 == 0 {
//             Color::RGB(205, 200, 190)
//         } else {
//             Color::RGB(180, 180, 170)
//         };
//         let square = Rect::new(
//             (col as u32 * SQUARE_SIZE) as i32,
//             (row as u32 * SQUARE_SIZE) as i32,
//             SQUARE_SIZE,
//             SQUARE_SIZE,
//         );
//         canvas.set_draw_color(color);
//         canvas.fill_rect(square)?;

//         // canvas.draw_rect(square)?;
//         let piece = if game.red.pieces.test_bit(i) {
//             if i == game.red.king {
//                 Some(&red_king)
//             } else {
//                 Some(&red_pawn)
//             }
//         } else if game.blue.pieces.test_bit(i) {
//             if i == game.blue.king {
//                 Some(&blue_king)
//             } else {
//                 Some(&blue_pawn)
//             }
//         } else {
//             None
//         };
//         if let Some(texture) = piece {
//             canvas.copy(texture, None, Some(square))?;
//         }
//     }

//     canvas.present();
//     std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
// }


// const BOARD_SIZE: u32 = 640;
// const SIDEBAR: u32 = 370;
// const SQUARE_SIZE: u32 = BOARD_SIZE / 5;
// const BACKGROUND_COLOR: Color = Color::RGB(20, 20, 20);
// const CARD_SQUARE_SIZE: u32 = 20;
    

// fn get_pos_from_click(x: i32, y: i32) -> Option<u8> {
//     let x = x as u32;
//     let y = y as u32;
//     if x > BOARD_SIZE {
//         None
//     } else {
//         let row = y / SQUARE_SIZE;
//         let col = x / SQUARE_SIZE;
//         Some((row * 5 + col) as u8)
//     }
// }
    