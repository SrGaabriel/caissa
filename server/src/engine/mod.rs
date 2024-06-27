use crate::board::{Piece, Pieces, PossibleMove};
use crate::board::board::ChessBoard;

pub mod minimax;

pub trait ChessEngine {
    fn new() -> Self;
    fn get_best_move(&mut self, board: &ChessBoard, depth: u32) -> PossibleMove;
}

pub fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Pieces::PAWN => 100,
        Pieces::BISHOP => 330,
        Pieces::KNIGHT => 320,
        Pieces::ROOK => 500,
        Pieces::QUEEN => 900,
        Pieces::KING => 10000,
        _ => panic!("Invalid piece")
    }
}