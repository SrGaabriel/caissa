use std::sync::{Arc, RwLock};

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::board::{BitBoard, BitPosition, GamePiece, MailBox, Pieces, Teams};
use crate::engine::ChessEngine;
use crate::engine::minimax::MinimaxEngine;
use crate::server::{get_best_move, get_piece_moves, get_team_moves, get_threatened_squares};

pub mod board;
pub mod game;
pub mod math;
pub mod engine;
pub mod hash;
mod server;

#[tokio::main]
async fn main() {
    let state = AppState {
        engine: MinimaxEngine::new(),
        depth: 6
    };
    let app = Router::new()
        .route("/api/playground/moves/best", post(get_best_move))
        .route("/api/playground/moves/team", post(get_team_moves))
        .route("/api/playground/moves/piece", post(get_piece_moves))
        .route("/api/playground/moves/threats", post(get_threatened_squares))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(Arc::new(RwLock::new(state))))
                .into_inner()
        );
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub struct AppState {
    pub engine: MinimaxEngine,
    pub depth: u8
}

type SharedState = Arc<RwLock<AppState>>;

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

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

// This method will print a BitPosition in teh same way printm prints a MailBox
// Instead of print (BitBoard), it will display the actual number of the pieces
pub fn printbp(bit_position: &BitPosition) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let mut piece = None;
            for team in 0..2 {
                for piece_type in 0..6 {
                    if bit_position.bb_pieces[team][piece_type].0 & (1 << index) != 0 {
                        piece = Some(GamePiece::from(piece_type, team));
                        break;
                    }
                }
                if piece.is_some() {
                    break;
                }
            }
            if let Some(piece) = piece {
                print!("{} ", piece.get_piece());
            } else {
                print!("- ");
            }
        }
        println!();
    }
}