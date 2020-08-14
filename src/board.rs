use std::sync::Arc;

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn to_pgn(&self) -> String {
        return match self {
            PieceType::King => String::from("K"),
            PieceType::Queen => String::from("Q"),
            PieceType::Rook => String::from("R"),
            PieceType::Bishop => String::from("B"),
            PieceType::Knight => String::from("N"),
            PieceType::Pawn => String::default(),
        };
    }
}

#[derive(Clone)]
pub struct Location {
    pub rank: u8,
    pub file: u8,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        let rankAlpha = ["1", "2", "3", "4", "5", "6", "7", "8"];
        let fileAlpha = ["a", "b", "c", "d", "e", "f", "g", "h"];
        return fileAlpha[self.file as usize].to_owned()
            + &rankAlpha[self.rank as usize].to_owned();
    }
}

pub struct PieceState {
    pub location: Location,
    pub has_moved: bool,
}

pub struct Piece {
    pub color: Color,
    pub r#type: PieceType,
}

pub struct BoardPiece {
    pub piece: Piece,
    pub state: PieceState,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        return match (&self.color, &self.r#type) {
            (Color::White, PieceType::King) => String::from("♔"),
            (Color::White, PieceType::Queen) => String::from("♕"),
            (Color::White, PieceType::Rook) => String::from("♖"),
            (Color::White, PieceType::Bishop) => String::from("♗"),
            (Color::White, PieceType::Knight) => String::from("♘"),
            (Color::White, PieceType::Pawn) => String::from("♙"),
            (Color::Black, PieceType::King) => String::from("♚"),
            (Color::Black, PieceType::Queen) => String::from("♛"),
            (Color::Black, PieceType::Rook) => String::from("♜"),
            (Color::Black, PieceType::Bishop) => String::from("♝"),
            (Color::Black, PieceType::Knight) => String::from("♞"),
            (Color::Black, PieceType::Pawn) => String::from("♟︎"),
        };
    }
}

pub struct Board {
    pub pieces: Vec<Arc<BoardPiece>>,
    pub squares: [[Option<Arc<BoardPiece>>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: vec![
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Rook,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 0 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Knight,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 1 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Bishop,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 2 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Queen,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 3 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::King,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 4 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Bishop,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 5 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Knight,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 6 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Rook,
                    },
                    state: PieceState {
                        location: Location { rank: 0, file: 7 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 0 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 1 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 2 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 3 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 4 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 5 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 6 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::White,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 1, file: 7 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Rook,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 0 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Knight,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 1 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Bishop,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 2 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Queen,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 3 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::King,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 4 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Bishop,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 5 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Knight,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 6 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Rook,
                    },
                    state: PieceState {
                        location: Location { rank: 7, file: 7 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 0 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 1 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 2 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 3 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 4 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 5 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 6 },
                        has_moved: false,
                    },
                }),
                Arc::new(BoardPiece {
                    piece: Piece {
                        color: Color::Black,
                        r#type: PieceType::Pawn,
                    },
                    state: PieceState {
                        location: Location { rank: 6, file: 7 },
                        has_moved: false,
                    },
                }),
            ],
            squares: Default::default(),
        };
        board.update_squares();
        return board;
    }

    fn update_squares(&mut self) {
        self.squares = Default::default();
        for piece in self.pieces.iter() {
            self.squares[piece.state.location.rank as usize][piece.state.location.file as usize] =
                Some(piece.clone());
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result: String = String::default();
        for rank in (0..8).rev() {
            for file in 0..8 {
                result += " ";
                if let Some(piece) = &self.squares[rank][file] {
                    result += &piece.piece.to_string();
                } else {
                    result += " ";
                }
                result += " ";
            }
            result += "\n\n";
        }
        return result;
    }
}
