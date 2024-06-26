use crate::board::{BitBoard, MailBox, Pieces, Teams};
use crate::game::fen;
use crate::math::kings::mask_king_castling_moves;

pub mod board;
pub mod game;
pub mod math;

#[tokio::main]
async fn main() {
    let board = fen::new_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/4K3 w KQkq - 0 1").unwrap();
    // print(&mask_king_castling_moves(Teams::WHITE, &!board.bits.empty_squares()));
    print(&board.optimistically_calculate_team_moves(Teams::WHITE));
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

pub fn printm(mail_box: &MailBox) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 16 + file;
            if let Some(piece) = mail_box.get_piece_at(index) {
                print!("{} ", piece);
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}