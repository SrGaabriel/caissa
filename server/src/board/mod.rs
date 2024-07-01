use std::ops::{BitAnd, BitOr, BitOrAssign, Div, Not, Rem};
use serde::Serialize;
use crate::game::Vector;

pub mod board;
pub mod state;

pub type Team = usize;
pub struct Teams;
impl Teams {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub fn get_opposite_team(team: Team) -> Team {
    if team == Teams::WHITE {
        Teams::BLACK
    } else {
        Teams::WHITE
    }
}

pub type Piece = usize;
pub struct Pieces;
impl Pieces{
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;

    pub fn iter() -> impl Iterator<Item = Piece> {
        0..6
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash, Ord)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum GamePiece {
    Pawn(Team),
    Bishop(Team),
    Knight(Team),
    Rook(Team),
    Queen(Team),
    King(Team)
}

impl GamePiece {
    pub fn from(piece: Piece, team: Team) -> GamePiece {
        match piece {
            Pieces::PAWN => GamePiece::Pawn(team),
            Pieces::BISHOP => GamePiece::Bishop(team),
            Pieces::KNIGHT => GamePiece::Knight(team),
            Pieces::ROOK => GamePiece::Rook(team),
            Pieces::QUEEN => GamePiece::Queen(team),
            Pieces::KING => GamePiece::King(team),
            _ => panic!("Invalid piece {}", piece)
        }
    }

    pub fn get_piece(&self) -> Piece {
        match self {
            GamePiece::Pawn(_) => Pieces::PAWN,
            GamePiece::Bishop(_) => Pieces::BISHOP,
            GamePiece::Knight(_) => Pieces::KNIGHT,
            GamePiece::Rook(_) => Pieces::ROOK,
            GamePiece::Queen(_) => Pieces::QUEEN,
            GamePiece::King(_) => Pieces::KING
        }
    }

    pub fn get_team(&self) -> Team {
        match self {
            GamePiece::Pawn(team) => *team,
            GamePiece::Bishop(team) => *team,
            GamePiece::Knight(team) => *team,
            GamePiece::Rook(team) => *team,
            GamePiece::Queen(team) => *team,
            GamePiece::King(team) => *team
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PossibleMove {
    pub origin: Vector,
    pub target: Vector
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PossibleMoves {
    pub origin: usize,
    pub attacks: BitBoard
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct BitPosition {
    bb_sides: [BitBoard; 2],
    pub(crate) bb_pieces: [[BitBoard; 6]; 2],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MailBox {
    board: [Option<GamePiece>; 128]
}

impl MailBox {
    pub fn new() -> Self {
        let board = [None; 128];
        // Initialize the board with pieces at their starting positions
        // ...
        Self { board }
    }

    pub fn get_piece_at(&self, index: usize) -> &Option<GamePiece> {
        &self.board[index]
    }

    pub fn set_piece_at(&mut self, index: usize, piece: Option<GamePiece>) {
        self.board[index] = piece;
    }

    pub fn is_valid_position(&self, index: usize) -> bool {
        index & 0x88 == 0
    }
}

pub fn mailbox_shift_up(index: usize, n: usize, team: Team) -> usize {
    if team == Teams::WHITE { index + n * 16 } else { index - n * 16 }
}

pub fn mailbox_shift_down(index: usize, n: usize, team: Team) -> usize {
    if team == Teams::WHITE { index - n * 16 } else { index + n * 16 }
}

impl BitPosition {
    pub fn new() -> Self {
        let bb_sides = [BitBoard(0), BitBoard(0)];
        let bb_pieces = [[BitBoard(0); 6]; 2];
        // Initialize the bitboards with pieces at their starting positions
        // ...
        Self { bb_sides, bb_pieces }
    }

    pub fn empty_squares(&self) -> BitBoard {
        !(self.bb_sides[Teams::BLACK] | self.bb_sides[Teams::WHITE])
    }

    pub fn move_or(&mut self, team: Team, piece: Piece, bitboard: BitBoard) {
        self.bb_pieces[team][piece] = self.bb_pieces[team][piece] | bitboard;
        self.bb_sides[team] = self.bb_sides[team] | bitboard;
    }

    pub fn move_and(&mut self, team: Team, piece: Piece, bitboard: BitBoard) {
        self.bb_pieces[team][piece] = self.bb_pieces[team][piece] & bitboard;
        self.bb_sides[team] = self.bb_sides[team] & bitboard;
    }

    pub fn get_pieces(&self, team: Team, piece: Piece) -> BitBoard {
        self.bb_pieces[team][piece]
    }

    pub fn get_team_pieces(&self, team: Team) -> BitBoard {
        self.bb_sides[team]
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

// Implementation for &BitBoard & BitBoard
impl<'a> BitAnd<BitBoard> for &'a BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

// Implementation for BitBoard & &BitBoard
impl<'a> BitAnd<&'a BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &'a BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

// Implementation for &BitBoard & &BitBoard
impl<'a, 'b> BitAnd<&'b BitBoard> for &'a BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: &'b BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl Rem for BitBoard {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 % rhs.0)
    }
}

impl Div for BitBoard {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 / rhs.0)
    }
}

impl BitBoard {
    pub fn shift_left(self, n: u32) -> Self {
        BitBoard(self.0 << n)
    }

    pub fn shift_right(self, n: u32) -> Self {
        BitBoard(self.0 >> n)
    }

    pub fn shift_up(self, n: u32, team: &Team) -> Self {
        if team == &Teams::WHITE {
            self.shift_left(n)
        } else {
            self.shift_right(n)
        }
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}

#[derive(Serialize, Debug, Hash, Clone)]
pub struct CompletedMove {
    pub origin: u8,
    pub target: u8,
    capture: u8,
    pub bits: u8,
}

impl CompletedMove {
    pub const PROMOTION: u8 = 0b0000_0001;
    pub const CASTLING: u8 = 0b0000_0010;
    pub const EN_PASSANT: u8 = 0b0000_0100;
    pub const CHECK: u8 = 0b0000_1000;
    pub const CHECKMATE: u8 = 0b0001_0000;
    pub const STALEMATE: u8 = 0b0010_0000;

    pub fn clean(origin: u8, target: u8) -> Self {
        Self { origin, target, bits: 0u8, capture: 0u8 }
    }

    pub fn new(origin: u8, target: u8, bits: u8, capture: u8) -> Self {
        Self { origin, target, bits, capture }
    }

    pub fn set_capture(&mut self, capture: Piece) {
        self.capture = (capture+1) as u8;
    }

    pub fn set_promotion(&mut self) {
        self.bits |= Self::PROMOTION;
    }

    pub fn set_castling(&mut self) {
        self.bits |= Self::CASTLING;
    }

    pub fn set_en_passant(&mut self) {
        self.bits |= Self::EN_PASSANT;
    }

    pub fn set_check(&mut self) {
        self.bits |= Self::CHECK;
    }

    pub fn set_checkmate(&mut self) {
        self.bits |= Self::CHECKMATE;
    }

    pub fn set_stalemate(&mut self) {
        self.bits |= Self::STALEMATE;
    }

    pub fn is_promotion(&self) -> bool {
        self.bits & Self::PROMOTION != 0
    }

    pub fn is_castling(&self) -> bool {
        self.bits & Self::CASTLING != 0
    }

    pub fn is_en_passant(&self) -> bool {
        self.bits & Self::EN_PASSANT != 0
    }

    pub fn is_check(&self) -> bool {
        self.bits & Self::CHECK != 0
    }

    pub fn is_checkmate(&self) -> bool {
        self.bits & Self::CHECKMATE != 0
    }

    pub fn is_stalemate(&self) -> bool {
        self.bits & Self::STALEMATE != 0
    }

    pub fn is_capture(&self) -> bool {
        self.capture != 0
    }

    pub fn get_capture(&self) -> Piece {
        (self.capture-1) as Piece
    }

    pub fn is_valid(&self) -> bool {
        self.origin < 64 && self.target < 64
    }
}