use serde::{Deserialize, Serialize};

pub mod engine;
pub mod moves;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum Team {
    White,
    Black
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Serialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub team: Team,
}

impl Piece {
    fn from_fen_char(c: char) -> Option<Piece> {
        let (piece_type, team) = match c {
            'K' => (PieceType::King, Team::White),
            'Q' => (PieceType::Queen, Team::White),
            'R' => (PieceType::Rook, Team::White),
            'B' => (PieceType::Bishop, Team::White),
            'N' => (PieceType::Knight, Team::White),
            'P' => (PieceType::Pawn, Team::White),
            'k' => (PieceType::King, Team::Black),
            'q' => (PieceType::Queen, Team::Black),
            'r' => (PieceType::Rook, Team::Black),
            'b' => (PieceType::Bishop, Team::Black),
            'n' => (PieceType::Knight, Team::Black),
            'p' => (PieceType::Pawn, Team::Black),
            _ => return None,
        };
        Some(Piece { piece_type, team })
    }
}

#[derive(Clone, Copy)]
pub struct BoardState {
    pub castling: CastlingRights,
    pub en_passant_target_square: Option<Coordinates>,
    pub team_to_play: Team,
    pub ending: Option<GameEnding>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum GameEnding {
    Checkmate,
    Stalemate,
}

#[derive(Clone, Copy)]
pub enum Side {
    KingSide,
    QueenSide,
}

#[derive(Clone, Copy, Serialize)]
pub struct Move {
    pub piece: Piece,
    pub from: Coordinates,
    pub to: Coordinates,
    pub check: bool,
    pub capture: bool,
    pub castle: bool,
    pub ending: Option<GameEnding>,
    pub en_passant: bool,
    pub promotion: bool,
}

fn get_opposite_team(team: Team) -> Team {
    match team {
        Team::White => Team::Black,
        Team::Black => Team::White,
    }
}

fn update_castling_state(state: &mut BoardState, team: Team, side: Side, value: bool) {
    match team {
        Team::White => match side {
            Side::KingSide => state.castling.white_king_side = value,
            Side::QueenSide => state.castling.white_queen_side = value,
        },
        Team::Black => match side {
            Side::KingSide => state.castling.black_king_side = value,
            Side::QueenSide => state.castling.black_queen_side = value,
        },
    }
}