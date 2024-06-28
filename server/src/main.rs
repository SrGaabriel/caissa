use std::sync::{Arc, RwLock};

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::board::{BitBoard, MailBox, Teams};
use crate::engine::ChessEngine;
use crate::engine::minimax::MinimaxEngine;
use crate::game::fen;
use crate::server::{get_best_move, get_piece_moves, get_team_moves};

pub mod board;
pub mod game;
pub mod math;
pub mod engine;
pub mod hash;
mod server;

#[tokio::main]
async fn main() {
    let state = AppState {
        engine: MinimaxEngine::new()
    };
    let app = Router::new()
        .route("/api/playground/moves/best", post(get_best_move))
        .route("/api/playground/moves/team", post(get_team_moves))
        .route("/api/playground/moves/piece", post(get_piece_moves))
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

#[derive(Clone)]
pub struct AppState {
    pub engine: MinimaxEngine
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