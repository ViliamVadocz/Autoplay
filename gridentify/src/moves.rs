use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Move {
    pub end: usize,
    pub used: HashSet<usize>,
}

impl Move {
    fn from(end: usize) -> Move {
        Move {
            end,
            used: HashSet::new(),
        }
    }
}

fn get_neighbours_of(board: &[u16; 25]) -> Vec<Vec<usize>> {
    let mut neighbours_of = Vec::new();
    for i in 0..25 {
        let x = i % 5;
        let y = i / 5;
        let value = board[i];
        let mut neighbours = Vec::new();
        // right
        if x < 4 && value == board[i + 1] {
            neighbours.push(i + 1);
        }
        // down
        if y < 4 && value == board[i + 5] {
            neighbours.push(i + 5);
        }
        // left
        if x > 0 && value == board[i - 1] {
            neighbours.push(i - 1);
        }
        // up
        if y > 0 && value == board[i - 5] {
            neighbours.push(i - 5);
        }
        neighbours_of.push(neighbours);
    }
    neighbours_of
}

pub fn possible_moves(board: &[u16; 25]) -> Vec<Move> {
    let neighbours_of = get_neighbours_of(board);
    let mut moves = Vec::new();

    // start moves at each tile
    for i in 0..25 {
        if !neighbours_of[i].is_empty() {
            explore(&Move::from(i), i, &neighbours_of, &mut moves)
        }
    }

    moves
}

fn explore(branch: &Move, pos: usize, neighbours_of: &[Vec<usize>], moves: &mut Vec<Move>) {
    // try expanding into each neighbour
    for &neighbour in neighbours_of[pos].iter() {
        // check that this tile is unexplored by this move
        if !(branch.end == neighbour || branch.used.contains(&neighbour)) {
            // branch off
            let mut new_branch = branch.clone();
            new_branch.used.insert(neighbour);
            // recursively explore
            explore(&new_branch, neighbour, neighbours_of, moves);
            moves.push(new_branch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::moves::possible_moves;

    #[test]
    fn dead_board() {
        let mut chequerboard = [0; 25];
        for (i, tile) in chequerboard.iter_mut().enumerate() {
            *tile = (i as u16 % 2) + 1;
        }
        let moves = possible_moves(&chequerboard);
        assert_eq!(moves.len(), 0)
    }

    #[test]
    fn layers() {
        let mut layers = [0; 25];
        for (i, tile) in layers.iter_mut().enumerate() {
            let layer = i as u16 / 5;
            *tile = (layer % 3) + 1;
        }
        let moves = possible_moves(&layers);
        assert_eq!(moves.len(), 100)
    }

    #[test]
    fn all_ones_board() {
        let moves = possible_moves(&[1; 25]);
        assert_eq!(moves.len(), 3060392)
    }
}
