use rand::Rng;

use crate::board::{Pieces, Teams};
use crate::board::board::ChessBoard;
use crate::board::state::CastlingSides;

pub struct ZobristHash {
    piece_keys: [[[u64; 64]; 6]; 2],
    turn_key: u64,
    castling_keys: [u64; 4], // 0: white can castle kingside, 1: white can castle queenside, 2: black can castle kingside, 3: black can castle queenside
    en_passant_keys: [u64; 8], // One for each file
    fifty_move_rule_key: u64,
}

impl ZobristHash {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut piece_keys = [[[0; 64]; 6]; 2];
        for team in 0..2 {
            for piece in 0..6 {
                for square in 0..64 {
                    piece_keys[team][piece][square] = rng.gen();
                }
            }
        }
        let turn_key = rng.gen();
        let mut castling_keys = [0; 4];
        for i in 0..4 {
            castling_keys[i] = rng.gen();
        }
        let mut en_passant_keys = [0; 8];
        for i in 0..8 {
            en_passant_keys[i] = rng.gen();
        }
        let fifty_move_rule_key = rng.gen();
        ZobristHash { piece_keys, turn_key, castling_keys, en_passant_keys, fifty_move_rule_key }
    }

    pub fn hash(&self, board: &ChessBoard) -> u64 {
        let mut hash = 0;
        for team in [Teams::WHITE, Teams::BLACK].iter() {
            for piece in [Pieces::PAWN, Pieces::KNIGHT, Pieces::BISHOP, Pieces::ROOK, Pieces::QUEEN, Pieces::KING].iter() {
                let bitboard = board.bits.get_pieces(*team, *piece);
                for square in 0..64 {
                    if bitboard.0 & (1 << square) != 0 {
                        hash ^= self.piece_keys[*team as usize][*piece as usize][square];
                    }
                }
            }
        }
        if board.state.team_to_play == Teams::WHITE {
            hash ^= self.turn_key;
        }
        if board.state.castling_rights.is_allowed(Teams::WHITE, CastlingSides::KINGSIDE) {
            hash ^= self.castling_keys[0];
        }
        if board.state.castling_rights.is_allowed(Teams::WHITE, CastlingSides::QUEENSIDE) {
            hash ^= self.castling_keys[1];
        }
        if board.state.castling_rights.is_allowed(Teams::BLACK, CastlingSides::KINGSIDE) {
            hash ^= self.castling_keys[2];
        }
        if board.state.castling_rights.is_allowed(Teams::BLACK, CastlingSides::QUEENSIDE) {
            hash ^= self.castling_keys[3];
        }
        if let Some(en_passant_square) = board.state.en_passant_square {
            hash ^= self.en_passant_keys[en_passant_square % 8];
        }
        // if board.state.halfmove_clock >= 50 {
        //     hash ^= self.fifty_move_rule_key;
        // }
        hash
    }
}