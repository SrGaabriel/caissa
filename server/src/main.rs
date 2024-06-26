use crate::board::{BitBoard, MailBox, Pieces, Teams};
use crate::game::fen;
use crate::math::knights::mask_all_knight_moves;
use crate::math::sliding::{properly_mask_all_bishop_moves, properly_mask_all_queen_moves, properly_mask_all_rook_moves, properly_mask_bishop_moves, properly_mask_rook_moves};

pub mod board;
pub mod game;
pub mod math;

#[tokio::main]
async fn main() {
    let board = fen::new_board("4r1k1/pp3p1p/2n2Qp1/3p2bq/1P1P2b1/P1N5/R3N1B1/5KR1 w - - 12 24").unwrap();
    print(&board.calculate_all_team_moves(Teams::WHITE));
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