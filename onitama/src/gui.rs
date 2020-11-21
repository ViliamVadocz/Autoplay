use crate::colour::Colour;
use crate::game::Move;
use crate::Transmission;

use std::result::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery, WindowCanvas};

use bitwise::TestBit;

// sizes
const FONT_SIZE: u16 = 28;
const BLOCK: u32 = 64;
const WIN_WIDTH: u32 = 19 * BLOCK;
const WIN_HEIGHT: u32 = 12 * BLOCK;
const BOARD_PAD: u32 = BLOCK;
const BOARD_SQUARE: u32 = 2 * BLOCK;
const BOARD_SIZE: u32 = 5 * BOARD_SQUARE;
const CARD_PAD: u32 = BLOCK;
const CARD_SQUARE: u32 = BLOCK / 2;
const CARD_SIZE: u32 = 5 * CARD_SQUARE;

// colour
const BG_COLOUR: Color = Color::RGB(20, 20, 20);
const W_SQUARE_COLOUR: Color = Color::RGB(239, 218, 182);
const B_SQUARE_COLOUR: Color = Color::RGB(179, 137, 101);
const SELECT_COLOUR: Color = Color::RGB(90, 150, 60);

macro_rules! rect {
    ($x:expr, $y:expr, $width:expr, $height:expr) => {
        Rect::new($x as i32, $y as i32, $width as u32, $height as u32)
    };
}

macro_rules! square {
    ($x:expr, $y:expr, $side:expr) => {
        rect!($x, $y, $side, $side)
    };
}

pub fn run(
    tx: Sender<Move>,
    rx: Receiver<Transmission>,
    should_end: &Arc<AtomicBool>,
) -> Result<(), String> {
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
    temple.set_color_mod(W_SQUARE_COLOUR.r, W_SQUARE_COLOUR.g, W_SQUARE_COLOUR.b);
    // load highlight and colour it (original image is white)
    let mut highlight = texture_creator.load_texture("./images/highlight.png")?;
    highlight.set_color_mod(SELECT_COLOUR.r, SELECT_COLOUR.g, SELECT_COLOUR.b);
    // load font
    let font = ttf_context.load_font("./fonts/Typographica-Blp5.ttf", FONT_SIZE)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = None;
    let mut want_move = false;
    let mut highlighted_squares = 0u32;
    let mut flipped = false;
    let mut red_username = None;
    let mut blue_username = None;
    'main_loop: loop {
        // early exit
        if should_end.load(Ordering::Relaxed) {
            break;
        }

        let mut clicked_square = None;

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
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    x,
                    y,
                    ..
                } => {
                    if let Some(pos) = get_pos_from_click(x as u32, y as u32) {
                        highlighted_squares ^= 1 << pos;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    flipped = !flipped;
                    highlighted_squares = highlighted_squares.reverse_bits() >> (32 - 25);
                }
                _ => {}
            }
        }

        // see if game can be updated
        if let Ok(trans) = rx.try_recv() {
            match trans {
                Transmission::Display(g) => game = Some(g),
                Transmission::RequestMove => want_move = true,
                Transmission::Usernames(red, blue) => {
                    // create username textures
                    let surface = font
                        .render(&red)
                        .blended(Color::WHITE)
                        .map_err(|e| e.to_string())?;
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;
                    red_username = Some(texture);
                    let surface = font
                        .render(&blue)
                        .blended(Color::WHITE)
                        .map_err(|e| e.to_string())?;
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;
                    blue_username = Some(texture);
                }
            }
        }

        // clear everything
        canvas.set_draw_color(BG_COLOUR);
        canvas.clear();

        // draw chequerboard
        match game {
            Some(ref actual_game) => {
                let (red, blue) = actual_game.get_red_blue();
                for mut pos in 0..25u8 {
                    let row = pos as u32 / 5;
                    let col = pos as u32 % 5;
                    let x = BOARD_PAD + BOARD_SQUARE * col;
                    let y = BOARD_PAD + BOARD_SQUARE * row;
                    let square = square!(x, y, BOARD_SQUARE);
                    canvas.set_draw_color(
                        if clicked_square.is_some() && clicked_square.unwrap() == pos as u32 {
                            SELECT_COLOUR
                        } else if pos % 2 == 0 {
                            B_SQUARE_COLOUR
                        } else {
                            W_SQUARE_COLOUR
                        },
                    );
                    canvas.fill_rect(square)?;
                    // add image (such as pieces or temple)
                    let p = if flipped { 24 - pos } else { pos };
                    if red.pieces.test_bit(p) {
                        if red.king == p {
                            canvas.copy(&red_king, None, Some(square))?;
                        } else {
                            canvas.copy(&red_pawn, None, Some(square))?;
                        }
                    } else if blue.pieces.test_bit(p) {
                        if blue.king == p {
                            canvas.copy(&blue_king, None, Some(square))?;
                        } else {
                            canvas.copy(&blue_pawn, None, Some(square))?;
                        }
                    } else if p == 2 || p == 22 {
                        canvas.copy(&temple, None, Some(square))?;
                    }
                    // add highlight
                    if highlighted_squares.test_bit(pos) {
                        canvas.copy(&highlight, None, Some(square))?;
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
                    let square = square!(x, y, BOARD_SQUARE);
                    canvas.set_draw_color(
                        if clicked_square.is_some() && clicked_square.unwrap() == pos {
                            SELECT_COLOUR
                        } else if pos % 2 == 0 {
                            B_SQUARE_COLOUR
                        } else {
                            W_SQUARE_COLOUR
                        },
                    );
                    canvas.fill_rect(square)?;
                    if pos == 2 || pos == 22 {
                        canvas.copy(&temple, None, Some(square))?;
                    }
                    if highlighted_squares.test_bit(pos) {
                        canvas.copy(&highlight, None, Some(square))?;
                    }
                }
            }
        }

        // draw cards
        if let Some(ref actual_game) = game {
            let (red, blue) = actual_game.get_red_blue();
            let (bottom, top) = if flipped { (blue, red) } else { (red, blue) };
            for (card, colour, start_x, start_y, text_flipped) in vec![
                (
                    &bottom.cards[0],
                    if flipped { Colour::Blue } else { Colour::Red },
                    BOARD_PAD + BOARD_SIZE + CARD_PAD,
                    WIN_HEIGHT - CARD_PAD - CARD_SIZE,
                    false,
                ),
                (
                    &bottom.cards[1],
                    if flipped { Colour::Blue } else { Colour::Red },
                    WIN_WIDTH - CARD_PAD - CARD_SIZE,
                    WIN_HEIGHT - CARD_PAD - CARD_SIZE,
                    false,
                ),
                (
                    &top.cards[0],
                    if flipped { Colour::Red } else { Colour::Blue },
                    BOARD_PAD + BOARD_SIZE + CARD_PAD,
                    CARD_PAD,
                    true,
                ),
                (
                    &top.cards[1],
                    if flipped { Colour::Red } else { Colour::Blue },
                    WIN_WIDTH - CARD_PAD - CARD_SIZE,
                    CARD_PAD,
                    true,
                ),
                (
                    &actual_game.table_card,
                    actual_game.colour,
                    WIN_WIDTH - CARD_PAD - CARD_SIZE,
                    (WIN_HEIGHT - CARD_SIZE) / 2,
                    matches!(actual_game.colour, Colour::Blue) ^ flipped,
                ),
            ]
            .into_iter()
            {
                let board = card.get_move(colour);
                for pos in 0..25 {
                    let row = pos / 5;
                    let col = pos % 5;
                    let x = start_x + CARD_SQUARE * col;
                    let y = start_y + CARD_SQUARE * row;
                    let square = square!(x, y, CARD_SQUARE);
                    let p = if flipped { 24 - pos } else { pos };
                    canvas.set_draw_color(if board.test_bit(p) {
                        SELECT_COLOUR
                    } else if pos % 2 == 0 {
                        B_SQUARE_COLOUR
                    } else {
                        W_SQUARE_COLOUR
                    });
                    canvas.fill_rect(square)?;
                    if pos == 2 || pos == 22 {
                        canvas.copy(&temple, None, Some(square))?;
                    }
                    if pos == 12 {
                        let pawn = match colour {
                            Colour::Red => &red_pawn,
                            Colour::Blue => &blue_pawn,
                        };
                        canvas.copy(pawn, None, Some(square))?;
                    }
                }

                let surface = font
                    .render(card.get_name())
                    .blended(Color::WHITE)
                    .map_err(|e| e.to_string())?;
                let name = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;
                let TextureQuery { width, height, .. } = name.query();
                let y = if text_flipped {
                    start_y + CARD_SIZE
                } else {
                    start_y - height
                };
                let text_rect = rect!(start_x, y, width, height);
                canvas.copy(&name, None, Some(text_rect))?;
            }
        }

        // write usernames
        if let Some(ref red) = red_username {
            let TextureQuery { width, height, .. } = red.query();
            let y = if flipped {
                BOARD_PAD - height
            } else {
                WIN_HEIGHT - BOARD_PAD
            };
            let text_rect = rect!(BOARD_PAD, y, width, height);
            canvas.copy(red, None, Some(text_rect))?;
        }
        if let Some(ref blue) = blue_username {
            let TextureQuery { width, height, .. } = blue.query();
            let y = if flipped {
                WIN_HEIGHT - BOARD_PAD
            } else {
                BOARD_PAD - height
            };
            let text_rect = rect!(BOARD_PAD, y, width, height);
            canvas.copy(blue, None, Some(text_rect))?;
        }

        canvas.present();
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
