use rand::Rng;
use crate::game::{PieceType, Team};
use crate::game::engine::BoardLogic; // Add this import at the top of your file

const BOARD_SIZE: usize = 8;
const NUM_PIECE_TYPES: usize = 6; // King, Queen, Rook, Bishop, Knight, Pawn
const NUM_TEAMS: usize = 2; // White, Black
const NUM_CASTLING_RIGHTS: usize = 4; // WhiteKingSide, WhiteQueenSide, BlackKingSide, BlackQueenSide
const NUM_EN_PASSANT_FILES: usize = 8; // Files a to h

// Define the Zobrist table
struct ZobristTable {
    pieces: [[[u64; BOARD_SIZE]; BOARD_SIZE]; NUM_PIECE_TYPES * NUM_TEAMS],
    castling: [u64; NUM_CASTLING_RIGHTS],
    en_passant: [u64; NUM_EN_PASSANT_FILES],
    side_to_move: u64,
}

impl ZobristTable {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut pieces = [[[0u64; BOARD_SIZE]; BOARD_SIZE]; NUM_PIECE_TYPES * NUM_TEAMS];
        for piece_type in 0..NUM_PIECE_TYPES {
            for team in 0..NUM_TEAMS {
                for x in 0..BOARD_SIZE {
                    for y in 0..BOARD_SIZE {
                        pieces[piece_type * NUM_TEAMS + team][x][y] = rng.gen();
                    }
                }
            }
        }

        let mut castling = [0u64; NUM_CASTLING_RIGHTS];
        for right in castling.iter_mut() {
            *right = rng.gen();
        }

        let mut en_passant = [0u64; NUM_EN_PASSANT_FILES];
        for file in en_passant.iter_mut() {
            *file = rng.gen();
        }

        let side_to_move = rng.gen();

        Self {
            pieces,
            castling,
            en_passant,
            side_to_move,
        }
    }
}

// Initialize the Zobrist table
lazy_static::lazy_static! {
    static ref ZOBRIST_TABLE: ZobristTable = ZobristTable::new();
}

// Implement the zobrist_hash function
impl BoardLogic {
    pub fn zobrist_hash(&self) -> u64 {
        let mut hash = 0u64;

        // Hash pieces
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if let Some(piece) = self.board[y][x] {
                    let piece_index = match piece.piece_type {
                        PieceType::King => 0,
                        PieceType::Queen => 1,
                        PieceType::Rook => 2,
                        PieceType::Bishop => 3,
                        PieceType::Knight => 4,
                        PieceType::Pawn => 5,
                    };
                    let team_index = match piece.team {
                        Team::White => 0,
                        Team::Black => 1,
                    };
                    hash ^= ZOBRIST_TABLE.pieces[piece_index * NUM_TEAMS + team_index][x][y];
                }
            }
        }

        // Hash castling rights
        if self.state.castling.white_king_side {
            hash ^= ZOBRIST_TABLE.castling[0];
        }
        if self.state.castling.white_queen_side {
            hash ^= ZOBRIST_TABLE.castling[1];
        }
        if self.state.castling.black_king_side {
            hash ^= ZOBRIST_TABLE.castling[2];
        }
        if self.state.castling.black_queen_side {
            hash ^= ZOBRIST_TABLE.castling[3];
        }

        // Hash en passant target square
        if let Some(en_passant) = self.state.en_passant_target_square {
            hash ^= ZOBRIST_TABLE.en_passant[en_passant.x as usize - 1];
        }

        // Hash side to move
        if self.state.team_to_play == Team::Black {
            hash ^= ZOBRIST_TABLE.side_to_move;
        }

        hash
    }
}
