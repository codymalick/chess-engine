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
    pub toMove: Player,
    pub board: Board<'a>,
    pub halfTurn: u8,
    pub fullTurn: u8,
    pub enPassante: &'a str
}

impl Game<'static> {
    pub fn new() -> Game<'static> {
        Game {
            toMove: Player::White,
            board: Board::new(),
            halfTurn: 0,
            fullTurn: 0,
            enPassante: "-"
        }
    }

    pub fn play(self) {
        print!("{}", self)
    }    
}

impl fmt::Display for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current Player: {}\n", self.toMove.to_string());
        write!(f, "Turn: {}\n", self.fullTurn);
        write!(f, "{}", self.board)
    }
}
