mod board;
mod fen;

fn main() {
    let mut board = board::Board::new();
    println!("{}", board);

    board.reset();
    println!("{}", board)
}
