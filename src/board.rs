use crate::fen::Fen;
use std::fmt;

#[derive(Debug)]
pub struct Board<'a> {
    pub previousStates: Vec<[&'a str; 64]>,
    pub state: [&'a str; 64],
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+---+---+---+---+---+\n").unwrap();
        for row in (0..=7).rev() {
            let offset = row * 8;
            write!(
                f,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {}\n",
                self.state[0 + offset],
                self.state[1 + offset],
                self.state[2 + offset],
                self.state[3 + offset],
                self.state[4 + offset],
                self.state[5 + offset],
                self.state[6 + offset],
                self.state[7 + offset],
                row + 1
            )
            .unwrap();
            write!(f, "+---+---+---+---+---+---+---+---+\n").unwrap();
        }
        write!(f, "  a   b   c   d   e   f   g   h  \n")
    }
}

impl Board<'_> {
    pub fn new() -> Self {
        Board {
            state: [" "; 64],
            previousStates: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.previousStates.push(self.state);
        self.state = [
            "R", "N", "B", "Q", "K", "B", "K", "R",
            "P", "P", "P", "P", "P", "P", "P", "P",
            " ", " ", " ", " ", " ", " ", " ", " ",
            " ", " ", " ", " ", " ", " ", " ", " ",
            " ", " ", " ", " ", " ", " ", " ", " ",
            " ", " ", " ", " ", " ", " ", " ", " ",
            "p", "p", "p", "p", "p", "p", "p", "p",
            "r", "n", "b", "q", "k", "b", "n", "r",
        ]
    }

    // fn from_fen(fen_string: &'static str) -> Board {
    //     let fen = Fen::new(fen_string);
    //     let state = Board::state_from_fen(&fen);
    //     Board { fen, state }
    // }

    // fn state_from_fen<'a>(fen: &'a Fen) -> [&'a str; 64] {
    //     let state = [" "; 64];
    //     for square in &fen.placement {
    //         println!("{}", square)
    //     }

    //     state
    // }
}
