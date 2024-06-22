use crate::board::{BitBoard, Pieces, Team, Teams};
use crate::board::hybrid::HybridChessBoard;
use crate::math::pawns::calculate_all_pawn_moves;

pub mod board;
pub mod game;
mod math;

#[tokio::main]
async fn main() {
    let board = HybridChessBoard::from_fen("rnb1k2B/1pp1qp2/4p2p/8/4Q1P1/bp6/P1PP1P1P/1K1R1BNR b q - 0 13").unwrap();
    let white_pawns = board.bit_position.get_pieces(Teams::BLACK, Pieces::PAWN);
    let pawn = board.mail_box.get_piece_at(21);
    println!("{:?}", pawn);
}


pub fn print(board: &BitBoard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            if board.0 & (1 << index) != 0 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}
