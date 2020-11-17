use crate::bot::get_move;
use crate::cards::Card;
use crate::cli::{Args, GameHost, Playing};
use crate::color::Color as GameColor;
use crate::connection::{Connection, Participant};
use crate::game::{Game, Move};
use crate::SERVER;

use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};

use bitwise::TestBit;

enum Transmission {
    Display(Game),
    RequestMove,
}

pub fn run(args: Args) -> Result<(), String> {
    let (tx_gui, rx_game) = channel();
    let (tx_game, rx_gui) = channel();
    // TODO also shut off original thread if gui thread is gone
    thread::spawn(move || gui(tx_gui, rx_gui).unwrap(/* TODO */));

    // helper closures
    let display = |game: &Game| {
        tx_game
            .send(Transmission::Display(game.clone()))
            .map_err(|e| e.to_string())
    };
    let get_move_from_gui = || {
        tx_game
            .send(Transmission::RequestMove)
            .map_err(|e| e.to_string())?;
        rx_game.recv().map_err(|e| e.to_string())
    };

    let (playing, host) = args;
    match host {
        GameHost::Local(mut game) => {
            let my_color = GameColor::Red; // TODO pick randomly
            while game.in_progress {
                display(&game)?;
                let the_move = if my_color == game.color {
                    // my turn
                    match playing {
                        Playing::Human => get_move_from_gui()?,
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    }
                } else {
                    // otherwise bot plays
                    get_move(&game)
                };
                game = game.take_turn(&the_move);
            }
        }
        GameHost::Online(maybe_match_id, username) => {
            let mut conn = Connection::new(SERVER)?;

            let (match_id, p) = match maybe_match_id {
                Some(match_id) => {
                    let p = if matches!(playing, Playing::No) {
                        // fake participant
                        Participant {
                            token: String::new(),
                            index: 0,
                        }
                    } else {
                        conn.join_match(&match_id, &username)
                    };
                    (match_id, p)
                }
                None => conn.create_match(&username),
            };
            println!("match id: {}", match_id);
            // println!("join: https://git.io/onitama#{}", match_id);
            // println!("spectate: https://git.io/onitama#spectate-{}", match_id);

            let mut state_msg = conn.spectate(&match_id);
            let color = if p.index == state_msg.indices.red {
                GameColor::Red
            } else {
                GameColor::Blue
            };
            let mut game = Game::from_state_msg(state_msg);
            while game.in_progress {
                display(&game)?;
                if color == game.color && !matches!(playing, Playing::No) {
                    let my_move = match playing {
                        Playing::Human => get_move_from_gui()?,
                        Playing::Bot => get_move(&game),
                        Playing::No => unreachable!(),
                    };
                    state_msg = conn.make_move(&my_move, &match_id, &p.token, &game);
                } else {
                    state_msg = conn.recv_state();
                }
                game = Game::from_state_msg(state_msg);
            }
        }
    };
    Ok(())
}

// sizes
const BLOCK: u32 = 64;
const WIN_WIDTH: u32 = 19 * BLOCK;
const WIN_HEIGHT: u32 = 12 * BLOCK;
const BOARD_PAD: u32 = BLOCK;
const BOARD_SQUARE: u32 = 2 * BLOCK;
const BOARD_SIZE: u32 = 5 * BOARD_SQUARE;
const CARD_PAD: u32 = BLOCK;
const CARD_SQUARE: u32 = BLOCK / 2;

// colour
const BG_COLOR: Color = Color::RGB(20, 20, 20);
const W_SQUARE_COLOR: Color = Color::RGB(205, 200, 190);
const B_SQUARE_COLOR: Color = Color::RGB(180, 180, 170);
const SELECT_COLOR: Color = Color::RGB(100, 200, 100);

macro_rules! rect {
    ($x:expr, $y:expr, $width:expr, $height:expr) => {
        Rect::new($x as i32, $y as i32, $width as u32, $height as u32)
    };
}

fn gui(tx: Sender<Move>, rx: Receiver<Transmission>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Onitama", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // load piece images
    let red_pawn = texture_creator.load_texture("./images/red_pawn.png")?;
    let red_king = texture_creator.load_texture("./images/red_king.png")?;
    let blue_pawn = texture_creator.load_texture("./images/blue_pawn.png")?;
    let blue_king = texture_creator.load_texture("./images/blue_king.png")?;
    // load temple and colour it (original image is white)
    let mut temple = texture_creator.load_texture("./images/temple.png")?;
    temple.set_color_mod(W_SQUARE_COLOR.r, W_SQUARE_COLOR.g, W_SQUARE_COLOR.b);
    // load font
    let font = ttf_context.load_font("./fonts/maturasc.ttf", 20)?;

    // let surface = font.render(card.get_name()).blended(Color::RED)?;
    // let texture = texture_creator.create_texture_from_surface(&surface)?;
    // let TextureQuery { width, height, .. } = texture.query();

    // canvas.fill_rect(rect)?;
    // canvas.set_draw_color(Color::BLACK);
    // canvas.draw_rect(rect)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut clicked_square = None;
    let mut game = None;
    let mut want_move = false;
    'main_loop: loop {
        // event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main_loop;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    clicked_square = get_pos_from_click(x as u32, y as u32);
                }
                // TODO right click to highlight
                // TODO F to flip view
                _ => {}
            }
        }

        // see if game can be updated
        let attempted_recv = rx.try_recv();
        if attempted_recv.is_ok() {
            match attempted_recv.unwrap() {
                Transmission::Display(g) => game = Some(g),
                Transmission::RequestMove => want_move = true,
            }
        }

        // clear everything
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();

        // draw chequerboard
        match game {
            Some(ref actual_game) => {
                let (red, blue) = actual_game.get_red_blue();
                for pos in 0..25u8 {
                    let row = pos as u32 / 5;
                    let col = pos as u32 % 5;
                    let x = BOARD_PAD + BOARD_SQUARE * col;
                    let y = BOARD_PAD + BOARD_SQUARE * row;
                    let square = rect!(x, y, BOARD_SQUARE, BOARD_SQUARE);
                    canvas.set_draw_color(
                        if clicked_square.is_some() && clicked_square.unwrap() == pos as u32 {
                            SELECT_COLOR
                        } else if pos % 2 == 0 {
                            B_SQUARE_COLOR
                        } else {
                            W_SQUARE_COLOR
                        },
                    );
                    canvas.fill_rect(square)?;
                    // add image (such as pieces or temple)
                    if red.pieces.test_bit(pos) {
                        if red.king == pos {
                            canvas.copy(&red_king, None, Some(square))?;
                        } else {
                            canvas.copy(&red_pawn, None, Some(square))?;
                        }
                    } else if blue.pieces.test_bit(pos) {
                        if blue.king == pos {
                            canvas.copy(&blue_king, None, Some(square))?;
                        } else {
                            canvas.copy(&blue_pawn, None, Some(square))?;
                        }
                    } else if pos == 2 || pos == 22 {
                        canvas.copy(&temple, None, Some(square))?;
                    }
                }
            }
            None => {
                // empty chequerboard while no game
                for pos in 0..25 {
                    let row = pos / 5;
                    let col = pos % 5;
                    let x = BOARD_PAD + BOARD_SQUARE * col;
                    let y = BOARD_PAD + BOARD_SQUARE * row;
                    let square = rect!(x, y, BOARD_SQUARE, BOARD_SQUARE);
                    canvas.set_draw_color(
                        if clicked_square.is_some() && clicked_square.unwrap() == pos {
                            SELECT_COLOR
                        } else if pos % 2 == 0 {
                            B_SQUARE_COLOR
                        } else {
                            W_SQUARE_COLOR
                        },
                    );
                    canvas.fill_rect(square)?;
                    if pos == 2 || pos == 22 {
                        canvas.copy(&temple, None, Some(square))?;
                    }
                }
            }
        }

        // TODO cards
        // TODO usernames

        canvas.present();
        // 60 fps POG
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn get_pos_from_click(x: u32, y: u32) -> Option<u32> {
    if (BOARD_PAD <= x && x < BOARD_PAD + BOARD_SIZE)
        && (BOARD_PAD <= y && y < BOARD_PAD + BOARD_SIZE)
    {
        let col = (x - BOARD_PAD) / BOARD_SQUARE;
        let row = (y - BOARD_PAD) / BOARD_SQUARE;
        Some(row * 5 + col)
    } else {
        None
    }
}
