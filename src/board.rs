use std::collections::HashMap;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

const FILE_CHARS: &'static str = "abcdefgh";

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Location {
    rank: u8,
    file: u8,
}

impl Location {
    fn move_relative(&self, color: Color, rank_shift: i8, file_shift: i8) -> Option<Self> {
        let rank_i8 = self.rank as i8;
        let file_i8 = self.file as i8;
        let new_rank = match color {
            Color::White => rank_i8 + rank_shift,
            Color::Black => rank_i8 - rank_shift,
        };
        let new_file = match color {
            Color::White => file_i8 + file_shift,
            Color::Black => file_i8 - file_shift,
        };
        if new_rank < 0 || new_rank > 7 || new_file < 0 || new_file > 7 {
            return None;
        }
        return Some(Self {
            rank: new_rank as u8,
            file: new_file as u8,
        });
    }

    fn forward(&self, color: Color) -> Option<Self> {
        return self.move_relative(color, 1, 0);
    }

    fn pgn(&self) -> String {
        return format!(
            "{}{}",
            FILE_CHARS
                .chars()
                .nth(self.file as usize)
                .expect(&format!("Invalid file {}", self.file)),
            self.rank + 1
        );
    }
}

#[derive(Debug)]
pub struct Diff {
    from: Option<Location>,
    to: Option<Location>,
    new_kind: Option<Kind>,
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::new();
        if let Some(from_loc) = self.from {
            result = format!("{}[from {}]", result, from_loc.pgn());
        }
        if let Some(to_loc) = self.to {
            result = format!("{}[to {}]", result, to_loc.pgn());
        }
        if let Some(new_kind) = &self.new_kind {
            result = format!("{}[as {:?}]", result, new_kind);
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub struct Move {
    diffs: Vec<Diff>,
}

impl Move {
    fn simple(from: Location, to: Location) -> Move {
        return Move {
            diffs: vec![Diff {
                from: Some(from),
                to: Some(to),
                new_kind: None,
            }],
        };
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}>", self.diffs)
    }
}

pub trait Piece {
    fn color(&self) -> Color;
    fn kind(&self) -> Kind;
    fn location(&self) -> Location;
    fn set_location(&mut self, location: Location);
    fn possible_moves(&self, board: &Board) -> Vec<Move>;
    fn repr(&self) -> &str;
}

struct Pawn {
    color: Color,
    location: Location,
}

impl Pawn {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::Pawn;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        println!("pawn moves");
        let mut result = vec![];
        if let Some(forward1) = self.location.forward(self.color) {
            println!("has forward 1");
            if let None = board.get_piece(&forward1) {
                println!("forward 1 unoccupied");
                if let Some(forward2) = forward1.forward(self.color) {
                    if let None = board.get_piece(&forward2) {
                        result.append(&mut vec![
                            Move::simple(self.location, forward1),
                            Move::simple(self.location, forward2),
                        ]);
                    } else {
                        result.append(&mut vec![Move::simple(self.location, forward1)]);
                    }
                } else {
                    result.append(&mut vec![Move::simple(self.location, forward1)]);
                }
            } else {
                println!("forward 1 occupied");
                println!(
                    "forward 1 loc: {:?} {:?}",
                    self.location.pgn(),
                    forward1.pgn()
                );
            }
        }
        return result;
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♙",
            Color::Black => "♟",
        };
    }
}

struct Knight {
    color: Color,
    location: Location,
}

impl Knight {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for Knight {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::Knight;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        return vec![];
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♘",
            Color::Black => "♞",
        };
    }
}

struct Bishop {
    color: Color,
    location: Location,
}

impl Bishop {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for Bishop {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::Bishop;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        return vec![];
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♗",
            Color::Black => "♝",
        };
    }
}

struct Rook {
    color: Color,
    location: Location,
}

impl Rook {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for Rook {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::Rook;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        return vec![];
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♖",
            Color::Black => "♜",
        };
    }
}

struct Queen {
    color: Color,
    location: Location,
}

impl Queen {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for Queen {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::Queen;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        return vec![];
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♕",
            Color::Black => "♛",
        };
    }
}

struct King {
    color: Color,
    location: Location,
}

impl King {
    fn new(color: Color, location: Location) -> Self {
        return Self {
            color: color,
            location: location,
        };
    }
}

impl Piece for King {
    fn color(&self) -> Color {
        return self.color;
    }

    fn kind(&self) -> Kind {
        return Kind::King;
    }

    fn location(&self) -> Location {
        return self.location;
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        return vec![];
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♔",
            Color::Black => "♚",
        };
    }
}

pub fn piece_from_repr(repr: &str, location: Location) -> Option<Box<dyn Piece>> {
    return match repr {
        "♔" => Some(Box::new(King::new(Color::White, location))),
        "♕" => Some(Box::new(Queen::new(Color::White, location))),
        "♖" => Some(Box::new(Rook::new(Color::White, location))),
        "♗" => Some(Box::new(Bishop::new(Color::White, location))),
        "♘" => Some(Box::new(Knight::new(Color::White, location))),
        "♙" => Some(Box::new(Pawn::new(Color::White, location))),
        "♚" => Some(Box::new(King::new(Color::Black, location))),
        "♛" => Some(Box::new(Queen::new(Color::Black, location))),
        "♜" => Some(Box::new(Rook::new(Color::Black, location))),
        "♝" => Some(Box::new(Bishop::new(Color::Black, location))),
        "♞" => Some(Box::new(Knight::new(Color::Black, location))),
        "♟" => Some(Box::new(Pawn::new(Color::Black, location))),
        " " => None,
        _ => panic!("Invalid repr character {}", repr),
    };
}

pub struct Board {
    pub pieces: HashMap<Location, Box<dyn Piece>>,
    pub to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: HashMap::new(),
            to_move: Color::White,
        };
        return board;
    }

    fn add_piece(&mut self, piece: Box<dyn Piece>) {
        assert!(!self.pieces.contains_key(&piece.as_ref().location()));
        self.pieces.insert(piece.as_ref().location(), piece);
        println!("{} pieces", self.pieces.len());
    }

    fn remove_piece(&mut self, location: &Location) {
        self.pieces.remove(location);
    }

    fn get_piece(&self, location: &Location) -> Option<&Box<dyn Piece>> {
        return self.pieces.get(location);
    }

    pub fn from_repr(repr: String) -> Self {
        assert_eq!(repr.graphemes(true).count(), 64);
        let mut board = Self::new();
        for r in (0..8).rev() {
            for f in 0..8 {
                let maybe_piece: Option<Box<dyn Piece>> = piece_from_repr(
                    repr.graphemes(true)
                        .nth((8 * (7 - r) + f) as usize)
                        .expect("Invalid board repr length"),
                    Location { rank: r, file: f },
                );
                if let Some(piece) = maybe_piece {
                    board.add_piece(piece);
                }
            }
        }
        return board;
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();
        for r in (0..8).rev() {
            for f in 0..8 {
                match self.get_piece(&Location { rank: r, file: f }) {
                    Some(piece) => result += piece.as_ref().repr(),
                    None => result += " ",
                };
                if f < 7 {
                    result += " ";
                }
            }
            result.push('\n');
        }
        return result;
    }

    pub fn default() -> Self {
        return Self::from_repr(String::from(concat!(
            "♜♞♝♛♚♝♞♜",
            "♟♟♟♟♟♟♟♟",
            "        ",
            "        ",
            "        ",
            "        ",
            "♙♙♙♙♙♙♙♙",
            "♖♘♗♕♔♗♘♖",
        )));
    }
    pub fn possible_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];
        for (_location, piece) in &self.pieces {
            if piece.color() == self.to_move {
                result.append(&mut piece.possible_moves(self));
            }
        }
        return result;
    }
}
