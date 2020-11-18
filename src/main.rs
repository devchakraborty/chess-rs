mod board;

fn main() {
    let board = board::Board::default();
    print!("{:?}", board.possible_moves());
}
