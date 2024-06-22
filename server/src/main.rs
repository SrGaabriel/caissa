use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::game::engine::BoardLogic;
use crate::server::{get_best_move, get_piece_moves, get_team_moves};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod game;
mod util;
mod server;
mod ai;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/playground/moves/best", post(get_best_move))
        .route("/api/playground/moves/team", post(get_team_moves))
        .route("/api/playground/moves/piece", post(get_piece_moves))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn print_board(board: &BoardLogic) {
    for y in (1..9).rev() {
        for x in 1..9 {
            let piece = board.get_piece_at(x, y);
            match piece {
                Some(p) => print!("{}", util::get_piece_letter(p.piece_type)),
                None => print!("."),
            }
        }
        println!();
    }
}