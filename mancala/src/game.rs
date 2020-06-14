use std::fmt;

pub enum Player {
    First,
    Second
}

pub enum Status {
    Running,
    Ended
}

pub struct Game {
    board: [u8; 14],
    pub current_player: Player,
    pub status: Status,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Status::Ended = self.status {
            let msg = match self.get_winner() {
                Some((Player::First, (p1, p2))) => format!("[{} : {}] First player won", p1, p2),
                Some((Player::Second, (p1, p2))) => format!("[{} : {}] Second player won", p1, p2),
                None => "It was a draw!".to_string()
            };
            return write!(f, "{}", msg);
        }

        let mut repr = String::new();

        // second player
        let top_row = self.board[7..14].iter().rev();
        let top_pad = top_row.clone().map(|x| x.to_string().len() < 2);
        for (num, pad) in top_row.zip(top_pad) {
            if pad {
                repr.push(' ');
            }
            repr.push(' ');
            repr.push_str(&num.to_string());
        }

        // alignment
        repr.push('\n');
        repr.push(' ');
        repr.push(' ');
        repr.push(' ');
            
        // first player
        let bottom_row = self.board[0..7].iter();
        let bottom_pad = bottom_row.clone().map(|x| x.to_string().len() < 2);
        for (num, pad) in bottom_row.zip(bottom_pad) {
            if pad {
                repr.push(' ');
            }
            repr.push(' ');
            repr.push_str(&num.to_string());
        }

        // who's turn is it
        match self.current_player {
            Player::First => repr.push_str("\n--- First player's turn  ---"),
            Player::Second => repr.push_str("\n--- Second player's turn ---")
        }
        
        write!(f, "{}", repr)
    }
}

impl Game {
    pub fn new(rocks: u8) -> Game {
        let mut board = [rocks; 14];
        // board representation
        // [_ C B A 9 8 7]
        // [0 1 2 3 4 5 _]
        board[6] = 0;
        board[13] = 0;
        Game {
            board,
            current_player: Player::First,
            status: Status::Running,
        }
    }
    
    pub fn make_move(&mut self, pit: usize) -> Result<(), &'static str> {
        if pit > 6 {
            return Err("pit out of range");
        }

        let (mut stones, mut index) = match self.current_player {
            Player::First => {
                let stones = self.board[pit];
                if stones == 0 {
                    return Err("The selected pit is empty");
                }
                (stones, pit)
            },
            Player::Second => {
                let stones = self.board[7 + pit];
                if stones == 0 {
                    return Err("The selected pit is empty");
                }
                (stones, 7 + pit)
            }
        };

        // take stones out
        self.board[index] = 0;
        while stones != 0 {
            // move along
            index += 1;
            index %= 14;
            // skip opponent's mancala
            if match self.current_player {
                Player::First => index == 13,
                Player::Second => index == 6
            } {
                continue;
            }
            // drop stone
            self.board[index] += 1;
            stones -= 1;
        }

        // switch current player except when
        // extra turn because landed in own mancala
        match self.current_player {
            Player::First => {
                if index != 6 {
                    self.current_player = Player::Second;
                }
            },
            Player::Second => {
                if index != 13 {
                    self.current_player = Player::First;
                }
            }
        }
        
        // handle capturing when last stone landed in
        // empty pit on own side
        if self.board[index] == 1 {
            match self.current_player {
                Player::First => {
                    if index < 6 {
                        let total = self.board[index + 7] + 1;
                        self.board[index] = 0;
                        self.board[index + 7] = 0;
                        self.board[6] += total;
                    }
                },
                Player::Second => {
                    if index > 6 && index < 13 {
                        let total = self.board[index - 7] + 1;
                        self.board[index] = 0;
                        self.board[index - 7] = 0;
                        self.board[13] += total;
                    }
                }
            }
        }

        // update status
        self.status = self.check_game_end();

        Ok(())
    }

    fn check_game_end(&self) -> Status {
        if self.board[0..6].iter().all(|&x| x == 0) || self.board[7..13].iter().all(|&x| x == 0) {
            Status::Ended
        } else {
            Status::Running
        }
    }

    fn get_winner(&self) -> Option<(Player, (u8, u8))> {
        match self.status {
            Status::Running => None,
            Status::Ended => {
                let f_stones: u8 = self.board[0..7].iter().sum();
                let s_stones: u8 = self.board[7..14].iter().sum();
                if f_stones > s_stones {
                    Some((Player::First, (f_stones, s_stones)))
                } else if s_stones > f_stones {
                    Some((Player::Second, (f_stones, s_stones)))
                } else {
                    None
                }
            }
        }
    }

    pub fn possible_moves(&self) -> Vec<usize> {
        let range = match self.current_player {
            Player::First => self.board[0..6].iter(),
            Player::Second => self.board[7..13].iter() //.rev()
        };
        range.enumerate().filter_map(|(i, &pit)| if pit != 0 {Some(i)} else {None}).collect()
    }
}
