use std::fmt;

use crate::board::Board;

pub enum Player {
    White,
    Black
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::White => write!(f, "W"),
            Player::Black => write!(f, "B"),
        }
    }
}

pub struct Game <'a> {
    pub to_move: Player,
    pub board: Board<'a>,
    pub half_turn: u8,
    pub full_turn: u8,
    pub en_passante: &'a str
}

impl Game<'static> {
    pub fn new() -> Game<'static> {
        Game {
            to_move: Player::White,
            board: Board::new(),
            half_turn: 0,
            full_turn: 0,
            en_passante: "-"
        }
    }

    pub fn play(self) {
        print!("{}", self)
    }    
}

impl fmt::Display for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Current Player: {}", self.to_move);
        writeln!(f, "Turn: {}", self.full_turn);
        writeln!(f, "{}", self.board)
    }
}
