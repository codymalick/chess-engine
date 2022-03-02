pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// FEN is composed of a few parts:
// A FEN record contains six fields. The separator between fields is a space. The fields are:[5]
// 1. Piece placement (from White's perspective). Each rank is described, starting with rank 8 and ending with rank 1; within each rank, the contents of each square are described from file "a" through file "h". Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the standard English names (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K"). White pieces are designated using upper-case letters ("PNBRQK") while black pieces use lowercase ("pnbrqk"). Empty squares are noted using digits 1 through 8 (the number of empty squares), and "/" separates ranks.
// 2. Active color. "w" means White moves next, "b" means Black moves next.
// 3. Castling availability. If neither side can castle, this is "-". Otherwise, this has one or more letters: "K" (White can castle kingside), "Q" (White can castle queenside), "k" (Black can castle kingside), and/or "q" (Black can castle queenside). A move that temporarily prevents castling does not negate this notation.
// 4. En passant target square in algebraic notation. If there's no en passant target square, this is "-". If a pawn has just made a two-square move, this is the position "behind" the pawn. This is recorded regardless of whether there is a pawn in position to make an en passant capture.[6]
// 5. Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[7]
// 6. Fullmove number: The number of the full move. It starts at 1, and is incremented after Black's move.

// Default game state:
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
// Example FEN after first move of the game, e4
// rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1
#[derive(Debug)]
#[allow(dead_code)]
pub struct Fen<'a> {
    pub placement: Vec<&'a str>,
    pub active_color: char,
    pub castling: &'a str,
    pub en_passant: &'a str,
    pub halfmove_timer: &'a [u8],
    pub fullmove_timer: &'a [u8],
}

impl Fen<'_> {
    pub fn new(input: &str) -> Fen {
        let split_fen: Vec<&str> = input.split(' ').collect();

        Fen {
            placement: split_fen[0].split('/').collect(),
            active_color: split_fen[1].chars().next().unwrap(),
            castling: split_fen[2],
            en_passant: split_fen[3],
            halfmove_timer: split_fen[4].as_bytes(),
            fullmove_timer: split_fen[5].as_bytes(),
        }
    }
}
