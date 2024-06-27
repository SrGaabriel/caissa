use std::time::Instant;

use crate::board::{BitBoard, MailBox};
use crate::engine::ChessEngine;
use crate::engine::minimax::MinimaxEngine;
use crate::game::fen;

pub mod board;
pub mod game;
pub mod math;
pub mod engine;
pub mod hash;

#[tokio::main]
async fn main() {
    let board = fen::new_board("6r1/1prk1p2/pn2pbp1/P2pP2p/1P1P3P/8/3N1PP1/1R4K1 b - - 0 29").unwrap();
    let mut engine = MinimaxEngine::new();
    let now = Instant::now();
    {
        let best_move = engine.get_best_move(&board, 3);
        println!("Best move: {:?}", best_move);
    }
    println!("Elapsed time: {:?}", now.elapsed());
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
                print!("{} ", piece.get_piece());
            } else {
                print!("- ");
            }
        }
        println!();
    }
}