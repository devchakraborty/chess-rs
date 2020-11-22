mod board;

use text_io::read;

fn main() {
    let mut board = board::Board::default();
    print!("{}", board.to_str());

    loop {
        let line: String = read!("{}\n");
        let r#move = board.parse_pgn_move(line.trim());
        println!("{:?}", r#move);
        board.apply_move(r#move);
        print!("{}", board.to_str());
    }
}
