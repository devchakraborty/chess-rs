mod board;

fn main() {
    let mut board = board::Board::default();
    board.apply_move(board::Move::Simple(
        board::Location { rank: 1, file: 4 },
        board::Location { rank: 3, file: 4 },
    ));
    print!("{}", board.to_str());
    print!("{:?}", board.possible_moves());
}
