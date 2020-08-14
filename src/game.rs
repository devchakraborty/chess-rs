use crate::board::{Board, BoardPiece, Color, Location, PieceType};
use std::sync::Arc;

pub enum MoveType {
    Normal,
    CastleQueenside,
    CastleKingside,
    Promotion(PieceType),
    Capture,
}

pub struct Move {
    piece: Arc<BoardPiece>,
    r#type: MoveType,
    from: Location,
    to: Location,
}

impl ToString for Move {
    fn to_string(&self) -> String {
        return self.piece.piece.r#type.to_pgn() + &self.to.to_string();
    }
}

pub struct Game {
    board: Board,
    turn: Color,
    history: Vec<Move>,
}

impl Game {
    pub fn new() -> Self {
        return Game {
            board: Board::new(),
            turn: Color::White,
            history: vec![],
        };
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for piece in &self.board.pieces {
            if piece.piece.color == self.turn {
                moves.extend(self.possible_moves_for_piece(&piece));
            }
        }
        return moves;
    }

    fn possible_moves_for_piece(&self, piece: &Arc<BoardPiece>) -> Vec<Move> {
        return match piece.piece.r#type {
            PieceType::Pawn => self.possible_moves_for_pawn(&piece),
            PieceType::Knight => vec![],
            PieceType::Bishop => vec![],
            PieceType::Rook => vec![],
            PieceType::Queen => vec![],
            PieceType::King => vec![],
        };
    }

    fn possible_moves_for_pawn(&self, piece: &Arc<BoardPiece>) -> Vec<Move> {
        assert!(piece.piece.r#type == PieceType::Pawn);
        let mut moves: Vec<Move> = vec![];
        let forward: i8 = match piece.piece.color {
            Color::White => 1,
            Color::Black => -1,
        };
        let promotion_rank: u8 = match piece.piece.color {
            Color::White => 7,
            Color::Black => 0,
        };
        assert!(piece.state.location.rank != promotion_rank);

        // Forward one square
        let forward_one_rank = ((piece.state.location.rank as i8) + forward) as u8;
        if let None =
            self.board.squares[forward_one_rank as usize][piece.state.location.file as usize]
        {
            if forward_one_rank == promotion_rank {
                // Pawn promotion
                moves.push(Move {
                    piece: piece.clone(),
                    r#type: MoveType::Promotion(PieceType::Knight),
                    from: piece.state.location.clone(),
                    to: Location {
                        rank: forward_one_rank,
                        file: piece.state.location.file,
                    },
                });
                moves.push(Move {
                    piece: piece.clone(),
                    r#type: MoveType::Promotion(PieceType::Bishop),
                    from: piece.state.location.clone(),
                    to: Location {
                        rank: forward_one_rank,
                        file: piece.state.location.file,
                    },
                });
                moves.push(Move {
                    piece: piece.clone(),
                    r#type: MoveType::Promotion(PieceType::Rook),
                    from: piece.state.location.clone(),
                    to: Location {
                        rank: forward_one_rank,
                        file: piece.state.location.file,
                    },
                });
                moves.push(Move {
                    piece: piece.clone(),
                    r#type: MoveType::Promotion(PieceType::Queen),
                    from: piece.state.location.clone(),
                    to: Location {
                        rank: forward_one_rank,
                        file: piece.state.location.file,
                    },
                });
            } else {
                // Forward one onto empty square
                moves.push(Move {
                    piece: piece.clone(),
                    r#type: MoveType::Normal,
                    from: piece.state.location.clone(),
                    to: Location {
                        rank: forward_one_rank,
                        file: piece.state.location.file,
                    },
                });

                if !piece.state.has_moved {
                    let forward_two_ranks: u8 = (forward_one_rank as i8 + forward) as u8;
                    if let None = self.board.squares[forward_two_ranks as usize]
                        [piece.state.location.file as usize]
                    {
                        moves.push(Move {
                            piece: piece.clone(),
                            r#type: MoveType::Normal,
                            from: piece.state.location.clone(),
                            to: Location {
                                rank: forward_two_ranks,
                                file: piece.state.location.file,
                            },
                        });
                    }
                }
            }
        }

        return moves;
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        return match self.turn {
            Color::White => "White to move:",
            Color::Black => "Black to move:",
        }
        .to_owned()
            + "\n"
            + &self.board.to_string();
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}
