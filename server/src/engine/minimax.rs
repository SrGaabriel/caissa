use std::collections::HashMap;
use crate::board::{CompletedMove, get_opposite_team, PossibleMove, Team};
use crate::board::board::ChessBoard;
use crate::engine::ChessEngine;
use crate::hash::ZobristHash;

#[derive(Clone)]
pub struct MinimaxEngine {
    zobrist: ZobristHash,
    transposition_table: HashMap<u64, i32>,
}

impl MinimaxEngine {
    fn minimax(&mut self, board: &mut ChessBoard, depth: u8, team: Team, alpha: i32, beta: i32) -> i32 {
        let maximizing = team == board.state.team_to_play;
        let zobrist = self.zobrist.hash(&board);

        if let Some(score) = self.transposition_table.get(&zobrist) {
            return *score;
        }

        if depth == 0 {
            let score = board.evaluate(team);
            self.transposition_table.insert(zobrist, score);
            return score;
        }

        let mut alpha = alpha;
        let mut beta = beta;

        if maximizing {
            let mut best_score = i32::MIN;
            for mv in board.generate_moves(team) {
                if let Some(mov) = board.play_move(mv.origin, mv.target, true) {
                    let score = -self.minimax(board, depth - 1, get_opposite_team(team), alpha, beta);
                    board.undo_move(&mov);
                    best_score = best_score.max(score);
                    alpha = alpha.max(best_score);
                    if beta <= alpha {
                        break; // Beta cut-off
                    }
                }
            }
            self.transposition_table.insert(zobrist, best_score);
            return best_score;
        } else {
            let mut best_score = i32::MAX;
            for mv in board.generate_moves(team) {
                if let Some(mov) = board.play_move(mv.origin, mv.target, true) {
                    let score = self.minimax(board, depth - 1, get_opposite_team(team), alpha, beta);
                    board.undo_move(&mov);
                    best_score = best_score.min(score);
                    beta = beta.min(best_score);
                    if beta <= alpha {
                        break; // Alpha cut-off
                    }
                }
            }
            self.transposition_table.insert(zobrist, best_score);
            return best_score;
        }
    }
}

impl ChessEngine for MinimaxEngine {
    fn new() -> Self {
        MinimaxEngine {
            zobrist: ZobristHash::new(),
            transposition_table: HashMap::new(),
        }
    }

    fn get_best_move(&mut self, board: &ChessBoard, depth: u8) -> CompletedMove {
        let mut board_clone = board.clone();
        let mut best_move = None;
        let mut best_score = i32::MIN;

        for mv in board.generate_moves(board.state.team_to_play) {
            if let Some(mov) = board_clone.play_move(mv.origin, mv.target, true) {
                let score = -self.minimax(&mut board_clone, depth - 1, get_opposite_team(board.state.team_to_play), i32::MIN, i32::MAX);
                board_clone.undo_move(&mov);
                println!("Move: {:?}, Score: {}", mv, score); // Debug log
                if score > best_score {
                    best_score = score;
                    best_move = Some(mov);
                }
            }
        }
        println!("Best move: {:?}, Best score: {}", best_move, best_score); // Debug log
        best_move.expect("No moves found")
    }
}
