use crate::moves::{get_fake_board, possible_boards, possible_moves, Move};

fn board_eval(board: &[u16; 25]) -> u16 {
    // temporary
    board.iter().sum::<u16>()
}

pub fn limited_const_depth_search(board: &[u16; 25], depth: u8) -> Move {
    possible_moves(board)
        .into_iter()
        .max_by_key(|my_move| {
            let fake_board = get_fake_board(*board, my_move);
            limited_tree_search(fake_board, 1, depth)
        })
        .unwrap()
}

fn limited_tree_search(board: [u16; 25], current_depth: u8, max_depth: u8) -> u16 {
    if current_depth >= max_depth {
        return board_eval(&board);
    }
    let moves = possible_moves(&board);
    if moves.is_empty() {
        return 0;
    }
    moves
        .into_iter()
        .map(|my_move| {
            let fake_board = get_fake_board(board, &my_move);
            limited_tree_search(fake_board, current_depth + 1, max_depth)
        })
        .max()
        .unwrap()
}

pub fn full_const_depth_search(board: &[u16; 25], depth: u8) -> Move {
    possible_moves(board)
        .into_iter()
        .max_by_key(|my_move| {
            let boards = possible_boards(*board, my_move);
            let board_evals = boards
                .into_iter()
                .map(|new_board| full_tree_search(new_board, 1, depth));
            match board_evals.min() {
                Some(score) => score,
                None => 0,
            }
        })
        .unwrap()
}

fn full_tree_search(board: [u16; 25], current_depth: u8, max_depth: u8) -> u16 {
    if current_depth >= max_depth {
        return board_eval(&board);
    }
    let moves = possible_moves(&board);
    if moves.is_empty() {
        return 0;
    }
    moves
        .into_iter()
        .map(|my_move| {
            let boards = possible_boards(board, &my_move);
            let board_evals = boards
                .into_iter()
                .map(|new_board| full_tree_search(new_board, current_depth + 1, max_depth));
            match board_evals.min() {
                Some(score) => score,
                None => 0,
            }
        })
        .max()
        .unwrap()
}

// fn choose_move(board: &[u16; 25]) -> Move {
//     let moves = possible_moves(&board);
// }
