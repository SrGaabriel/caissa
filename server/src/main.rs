use crate::board::{BitBoard, MailBox, Pieces, Team, Teams};
use crate::math::knights::calculate_all_knight_moves;
use crate::game::fen;

pub mod board;
pub mod game;
pub mod math;

#[tokio::main]
async fn main() {
    let board = fen::new_board("rnb1k2B/1pp1qp2/4p2p/8/4Q1P1/bp6/P1PP1P1P/1K1R1BNR b q - 0 13").unwrap();
    print(&calculate_all_knight_moves(&board.bits, Teams::BLACK));
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