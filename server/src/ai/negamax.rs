use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash};
use crate::ai::ChessAI;
use crate::game::engine::BoardLogic;
use crate::game::Move;

pub struct Negamax {
    transposition_table: HashMap<u64, (i32, i32, i32)>, // Store hash -> (depth, alpha, beta)
}

impl Negamax {
    pub fn new() -> Negamax {
        Negamax {
            transposition_table: HashMap::new(),
        }
    }

    fn negamax(&mut self, board: BoardLogic, depth: i32, mut alpha: i32, mut beta: i32, color: i32) -> i32 {
        // let hash = board.hash(&mut DefaultHasher::new()); // Hash the board position
        // if let Some((cached_depth, cached_alpha, cached_beta)) = self.transposition_table.get(&hash) {
        //     if *cached_depth >= depth {
        //         if *cached_alpha >= beta {
        //             return *cached_alpha;
        //         }
        //         if *cached_beta <= alpha {
        //             return *cached_beta;
        //         }
        //         alpha = alpha.max(*cached_alpha);
        //         beta = beta.min(*cached_beta);
        //     }
        // }

        if depth == 0 {
            let score = color * board.evaluate_position(board.state.team_to_play);
            // self.transposition_table.insert(hash, (depth, score, score)); // Cache the position
            return score;
        }

        let mut max_eval = i32::MIN + 1;
        for mov in board.get_all_moves_for_team(board.state.team_to_play) { // Implement move ordering
            let mut new_board = board.clone();
            new_board.play_move(mov.from.x, mov.from.y, mov.to.x, mov.to.y, false, true);
            let eval = -self.negamax(new_board, depth - 1, -beta, -alpha, -color);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if alpha >= beta {
                break;
            }
        }

        // self.transposition_table.insert(hash, (depth, alpha, beta)); // Cache the position
        max_eval
    }
}

impl ChessAI for Negamax {
    fn get_best_move(&mut self, board: BoardLogic, depth: i32) -> Option<Move> {
        let mut best_move: Option<Move> = None;
        let mut max_eval = i32::MIN + 1;
        let alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;
        for mov in board.get_all_moves_for_team(board.state.team_to_play) { // Implement move ordering
            let mut new_board = board.clone();
            new_board.play_move(mov.from.x, mov.from.y, mov.to.x, mov.to.y, false, true);
            let eval = -self.negamax(new_board, depth - 1, alpha, beta, -1);
            if eval > max_eval {
                max_eval = eval;
                best_move = Some(mov);
            }
        }
        best_move
    }
}
