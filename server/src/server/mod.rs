use std::thread;

use axum::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::board::{CompletedMove, get_opposite_team, PossibleMove};
use crate::engine::ChessEngine;
use crate::game::{fen, Vector};
use crate::math::map_bits;
use crate::SharedState;

pub async fn get_team_moves(
    Json(payload): Json<TeamMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = fen::new_board(&payload.fen);
    if board.is_none() {
        return (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
    }
    let board = board.unwrap();
    let moves = board.generate_moves(payload.team as usize);
    (StatusCode::OK, Json(Moves { moves }))
}

pub async fn get_piece_moves(
    Json(payload): Json<PieceMovesRequest>
) -> (StatusCode, Json<Moves>) {
    let board = fen::new_board(&payload.fen);
    if board.is_none() {
        return (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
    }
    let board = board.unwrap();
    let moves = board.calculate_move_for_piece(payload.coordinates);
    (StatusCode::BAD_REQUEST, Json(Moves { moves: Vec::new() }))
}

pub async fn get_threatened_squares(
    Json(payload): Json<BestMoveRequest>
) -> (StatusCode, Json<ThreatsResponse>) {
    let board = fen::new_board(&payload.fen);
    if board.is_none() {
        return (StatusCode::BAD_REQUEST, Json(ThreatsResponse { threats: Vec::new() }))
    }
    let board = board.unwrap();
    let threats = board.generate_moves(board.state.team_to_play).iter().map(|mv| {
        mv.target as u32
    }).collect();
    (StatusCode::OK, Json(ThreatsResponse { threats }))
}

pub async fn get_best_move(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<BestMoveRequest>,
) -> Result<Json<CompletedMove>, StatusCode> {
    let board = fen::new_board(&payload.fen);
    let thread = thread::Builder::new()
        .name("negamax".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let mut state = &mut state.write().unwrap();
            let depth = state.depth.clone();
            state.engine.get_best_move(&board.unwrap(), depth)
        }).unwrap();
    let best_move = thread.join().unwrap();
    Ok(Json(best_move))
}

#[derive(Deserialize)]
pub struct TeamMovesRequest {
    fen: String,
    team: u8
}

#[derive(Deserialize)]
pub struct BestMoveRequest {
    fen: String
}

#[derive(Deserialize)]
pub struct PieceMovesRequest {
    fen: String,
    coordinates: Vector
}

#[derive(Serialize)]
pub struct Moves {
    moves: Vec<CompletedMove>
}

#[derive(Serialize)]
pub struct ThreatsResponse {
    threats: Vec<u32>
}