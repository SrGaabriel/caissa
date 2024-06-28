use rand::Rng;
use crate::board::{Pieces, Team, Teams};
use crate::board::board::ChessBoard;
use crate::board::state::CastlingSides;

#[derive(Clone)]
pub struct ZobristHash {
    piece_keys: [[[u64; 64]; 6]; 2],
    turn_key: u64,
    castling_keys: [u64; 4],
    en_passant_keys: [u64; 8],
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
        for key in &mut castling_keys {
            *key = rng.gen();
        }
        let mut en_passant_keys = [0; 8];
        for key in &mut en_passant_keys {
            *key = rng.gen();
        }
        let fifty_move_rule_key = rng.gen();
        ZobristHash { piece_keys, turn_key, castling_keys, en_passant_keys, fifty_move_rule_key }
    }

    pub fn hash(&self, board: &ChessBoard) -> u64 {
        let mut hash = 0;

        for (team, piece_bits) in board.bits.bb_pieces.iter().enumerate() {
            for (piece, bitboard) in piece_bits.iter().enumerate() {
                let mut bits = bitboard.0;
                while bits != 0 {
                    let square = bits.trailing_zeros() as usize;
                    hash ^= self.piece_keys[team][piece][square];
                    bits &= bits - 1;
                }
            }
        }

        if board.state.team_to_play == Teams::WHITE {
            hash ^= self.turn_key;
        }

        let castling_rights = &board.state.castling_rights;
        if castling_rights.is_allowed(Teams::WHITE, CastlingSides::KINGSIDE) {
            hash ^= self.castling_keys[0];
        }
        if castling_rights.is_allowed(Teams::WHITE, CastlingSides::QUEENSIDE) {
            hash ^= self.castling_keys[1];
        }
        if castling_rights.is_allowed(Teams::BLACK, CastlingSides::KINGSIDE) {
            hash ^= self.castling_keys[2];
        }
        if castling_rights.is_allowed(Teams::BLACK, CastlingSides::QUEENSIDE) {
            hash ^= self.castling_keys[3];
        }

        if let Some(en_passant_square) = board.state.en_passant_square {
            hash ^= self.en_passant_keys[(en_passant_square % 8) as usize];
        }

        // Uncomment if fifty_move_rule_key is needed
        // if board.state.halfmove_clock >= 50 {
        //     hash ^= self.fifty_move_rule_key;
        // }

        hash
    }
}
