use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

const FILE_CHARS: &'static str = "abcdefgh";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

fn kind_to_pgn(kind: &Kind) -> String {
    String::from(match kind {
        Kind::King => "K",
        Kind::Queen => "Q",
        Kind::Rook => "R",
        Kind::Bishop => "B",
        Kind::Knight => "N",
        Kind::Pawn => "",
    })
}

fn pgn_to_kind(pgn: &str) -> Kind {
    match pgn {
        "K" => Kind::King,
        "Q" => Kind::Queen,
        "R" => Kind::Rook,
        "B" => Kind::Bishop,
        "N" => Kind::Knight,
        "" => Kind::Pawn,
        _ => panic!("Invalid piece kind: {}", pgn),
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Location {
    pub rank: u8,
    pub file: u8,
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

    fn left(&self, color: Color) -> Option<Self> {
        return self.move_relative(color, 0, -1);
    }

    fn right(&self, color: Color) -> Option<Self> {
        return self.move_relative(color, 0, 1);
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

    fn parse_rank(rank: &str) -> u8 {
        let result = rank.parse::<u8>().unwrap() - 1;
        assert!(result >= 0 && result < 8, format!("Invalid rank: {}", rank));
        return result;
    }

    fn parse_file(file: &str) -> u8 {
        assert!(file.len() == 1, format!("Invalid file: {}", file));
        FILE_CHARS
            .find(file)
            .expect(&format!("Invalid file: {}", file)) as u8
    }

    fn parse_pgn(pgn: &str) -> Location {
        assert!(pgn.len() == 2, format!("Invalid square: {}", pgn));
        Location {
            rank: Location::parse_rank(&pgn[1..2]),
            file: Location::parse_file(&pgn[0..1]),
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    from: Option<Location>,
    to: Option<Location>,
    new_kind: Option<Kind>,
}

#[derive(Debug)]
pub enum Move {
    Simple(Location, Location),
}

pub trait Piece: core::fmt::Debug {
    fn color(&self) -> Color;
    fn kind(&self) -> Kind;
    fn location(&self) -> Location;
    fn set_location(&mut self, location: Location);
    fn possible_moves(&self, board: &Board) -> Vec<Move>;
    fn repr(&self) -> &str;
}

#[derive(Debug)]
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
        let mut result = vec![];
        if let Some(forward1) = self.location.forward(self.color) {
            if let None = board.get_piece(&forward1) {
                if let Some(forward2) = forward1.forward(self.color) {
                    if let None = board.get_piece(&forward2) {
                        result.append(&mut vec![
                            Move::Simple(self.location, forward1),
                            Move::Simple(self.location, forward2),
                        ]);
                    } else {
                        result.push(Move::Simple(self.location, forward1));
                    }
                } else {
                    result.push(Move::Simple(self.location, forward1));
                }
            }

            if let Some(capture_left1) = forward1.left(self.color) {
                if let Some(capture_left_piece1) = board.get_piece(&capture_left1) {
                    if capture_left_piece1.color() != self.color {
                        result.push(Move::Simple(self.location, capture_left1));
                    }
                }
            }

            if let Some(capture_right1) = forward1.right(self.color) {
                if let Some(capture_right_piece1) = board.get_piece(&capture_right1) {
                    if capture_right_piece1.color() != self.color {
                        result.push(Move::Simple(self.location, capture_right1));
                    }
                }
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

#[derive(Debug)]
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
        let mut result: Vec<Move> = vec![];
        let offsets: [(i8, i8); 8] = [
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];
        for offset in offsets.iter() {
            if let Some(location) = self.location.move_relative(self.color, offset.0, offset.1) {
                if let Some(piece) = board.get_piece(&location) {
                    if piece.color() != self.color {
                        result.push(Move::Simple(self.location, location));
                    }
                } else {
                    result.push(Move::Simple(self.location, location));
                }
            }
        }
        return result;
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♘",
            Color::Black => "♞",
        };
    }
}

#[derive(Debug)]
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
        let directions: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        let mut result: Vec<Move> = vec![];

        for direction in directions.iter() {
            for distance in 1..8 {
                if let Some(target_location) = self.location.move_relative(
                    self.color,
                    distance * direction.0,
                    distance * direction.1,
                ) {
                    if let Some(target_piece) = board.get_piece(&target_location) {
                        if target_piece.color() != self.color {
                            result.push(Move::Simple(self.location, target_location));
                        }
                        break;
                    } else {
                        result.push(Move::Simple(self.location, target_location));
                    }
                }
            }
        }
        return result;
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♗",
            Color::Black => "♝",
        };
    }
}

#[derive(Debug)]
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
        let directions: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result: Vec<Move> = vec![];

        for direction in directions.iter() {
            for distance in 1..8 {
                if let Some(target_location) = self.location.move_relative(
                    self.color,
                    distance * direction.0,
                    distance * direction.1,
                ) {
                    if let Some(target_piece) = board.get_piece(&target_location) {
                        if target_piece.color() != self.color {
                            result.push(Move::Simple(self.location, target_location));
                        }
                        break;
                    } else {
                        result.push(Move::Simple(self.location, target_location));
                    }
                }
            }
        }
        return result;
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♖",
            Color::Black => "♜",
        };
    }
}

#[derive(Debug)]
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
        let directions: [(i8, i8); 8] = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        let mut result: Vec<Move> = vec![];

        for direction in directions.iter() {
            for distance in 1..8 {
                if let Some(target_location) = self.location.move_relative(
                    self.color,
                    distance * direction.0,
                    distance * direction.1,
                ) {
                    if let Some(target_piece) = board.get_piece(&target_location) {
                        if target_piece.color() != self.color {
                            result.push(Move::Simple(self.location, target_location));
                        }
                        break;
                    } else {
                        result.push(Move::Simple(self.location, target_location));
                    }
                }
            }
        }
        return result;
    }

    fn repr(&self) -> &str {
        return match self.color {
            Color::White => "♕",
            Color::Black => "♛",
        };
    }
}

#[derive(Debug)]
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
        return Board {
            pieces: HashMap::new(),
            to_move: Color::White,
        };
    }

    fn add_piece(&mut self, piece: Box<dyn Piece>) {
        self.pieces.insert(piece.as_ref().location(), piece);
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
    pub fn apply_move(&mut self, r#move: Move) {
        match r#move {
            Move::Simple(from, to) => {
                let mut piece = self
                    .pieces
                    .remove(&from)
                    .expect(&format!("No piece at {}", from.pgn()));
                piece.set_location(to);
                self.pieces.insert(to, piece);
            }
        }
        self.to_move = match &self.to_move {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
    }

    pub fn to_pgn(&self, r#move: &Move) -> String {
        match r#move {
            Move::Simple(from, to) => {
                if let Some(move_piece) = self.get_piece(from) {
                    let mut result = String::new();
                    result.push_str(&kind_to_pgn(&move_piece.as_ref().kind()));
                    result.push_str(&to.pgn());
                    return result;
                } else {
                    panic!("No piece at {}", from.pgn());
                }
            }
        }
    }

    pub fn parse_pgn_move(&self, pgn: &str) -> Move {
        let simple_re: Regex = Regex::new(r"^([NBRQK]?)([a-h]?)([1-8]?)x?([a-h][1-8])$").unwrap();
        if simple_re.is_match(pgn) {
            let cap = simple_re.captures_iter(pgn).next().expect("");
            let kind = pgn_to_kind(&cap[1]);
            let source_file: Option<u8> = match &cap[2] {
                "" => None,
                file_char => Some(FILE_CHARS.find(file_char).expect("") as u8),
            };
            let source_rank: Option<u8> = match &cap[3] {
                "" => None,
                rank_char => Some(rank_char.parse::<u8>().unwrap() - 1),
            };
            let dest_loc = Location::parse_pgn(&cap[4]);
            let mut candidate_pieces: Vec<&Box<dyn Piece>> = vec![];
            for (_, piece) in &self.pieces {
                if piece.color() != self.to_move {
                    continue;
                }
                if piece.kind() != kind {
                    continue;
                }
                if let Some(from_file) = source_file {
                    if from_file != piece.location().file {
                        continue;
                    }
                }
                if let Some(from_rank) = source_rank {
                    if from_rank != piece.location().rank {
                        continue;
                    }
                }
                let mut can_move = false;
                for r#move in piece.possible_moves(self) {
                    if let Move::Simple(from, to) = r#move {
                        if to == dest_loc {
                            can_move = true;
                            break;
                        }
                    }
                }
                if !can_move {
                    continue;
                }
                candidate_pieces.push(piece);
            }
            println!("{:?}", candidate_pieces);
            assert!(
                candidate_pieces.len() > 0,
                format!("No pieces can make the move: {}", pgn)
            );
            assert!(
                candidate_pieces.len() == 1,
                format!("Move is ambiguous: {}", pgn)
            );
            let move_piece = candidate_pieces[0];
            return Move::Simple(move_piece.location(), dest_loc);
        }
        panic!("Could not parse move: {}", pgn);
    }
}
