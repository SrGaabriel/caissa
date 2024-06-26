use crate::board::{BitBoard, BitPosition, Pieces, Team, Teams};
use crate::board::state::{CastlingRights, CastlingSides};

pub const WHITE_QUEENSIDE_KING_CASTLE: u64 = 0x4;
pub const WHITE_KINGSIDE_KING_CASTLE: u64 = 0x40;
pub const BLACK_QUEENSIDE_KING_CASTLE: u64 = 0x400000000000000;
pub const BLACK_KINGSIDE_KING_CASTLE: u64 = 0x4000000000000000;
pub const WHITE_QUEENSIDE_CASTLE_BLOCKS: u64 = 0xE;
pub const WHITE_KINGSIDE_CASTLE_BLOCKS: u64 = 0x60;
pub const BLACK_QUEENSIDE_CASTLE_BLOCKS: u64 = 0xE00000000000000;
pub const BLACK_KINGSIDE_CASTLE_BLOCKS: u64 = 0x6000000000000000;

pub fn calculate_king_moves(position: &BitPosition, team: Team) -> BitBoard {
    let king = position.get_pieces(team, Pieces::KING);
    let king_moves = mask_king_moves(king.0);
    king_moves
}

pub fn mask_king_moves(king_bit: u64) -> BitBoard {
    let not_a_file = 0xfefefefefefefefe;
    let not_h_file = 0x7f7f7f7f7f7f7f7f;

    let mut moves = 0;
    if (king_bit >> 1) & not_h_file != 0 { moves |= king_bit >> 1; }
    if (king_bit << 1) & not_a_file != 0 { moves |= king_bit << 1; }
    if king_bit >> 8 != 0 { moves |= king_bit >> 8; }
    if king_bit << 8 != 0 { moves |= king_bit << 8; }
    if (king_bit >> 9) & not_h_file != 0 { moves |= king_bit >> 9; }
    if (king_bit << 7) & not_a_file != 0 { moves |= king_bit << 7; }
    if (king_bit >> 7) & not_a_file != 0 { moves |= king_bit >> 7; }
    if (king_bit << 9) & not_h_file != 0 { moves |= king_bit << 9; }

    BitBoard(moves)
}

pub fn calculate_king_castling_moves(castling_rights: &CastlingRights, team: Team, occupied_spaces: &BitBoard) -> BitBoard {
    let mut castling_moves = BitBoard(0);

    if team == Teams::WHITE && castling_rights.is_allowed(Teams::WHITE, CastlingSides::KINGSIDE) && (occupied_spaces.0 & WHITE_KINGSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= WHITE_KINGSIDE_KING_CASTLE;
    } else if team == Teams::BLACK && castling_rights.is_allowed(Teams::BLACK, CastlingSides::KINGSIDE) && (occupied_spaces.0 & BLACK_KINGSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= BLACK_KINGSIDE_KING_CASTLE;
    }
    if team == Teams::WHITE && castling_rights.is_allowed(Teams::WHITE, CastlingSides::QUEENSIDE) && (occupied_spaces.0 & WHITE_QUEENSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= WHITE_QUEENSIDE_KING_CASTLE;
    } else if team == Teams::BLACK && castling_rights.is_allowed(Teams::BLACK, CastlingSides::QUEENSIDE) && (occupied_spaces.0 & BLACK_QUEENSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= BLACK_QUEENSIDE_KING_CASTLE;
    }

    castling_moves
}

pub fn mask_king_castling_moves(team: Team, occupied_spaces: &BitBoard) -> BitBoard {
    let mut castling_moves = BitBoard(0);

    if team == Teams::WHITE && (occupied_spaces.0 & WHITE_KINGSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= WHITE_KINGSIDE_KING_CASTLE;
    } else if team == Teams::BLACK && (occupied_spaces.0 & BLACK_KINGSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= BLACK_KINGSIDE_KING_CASTLE;
    }
    if team == Teams::WHITE && (occupied_spaces.0 & WHITE_QUEENSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= WHITE_QUEENSIDE_KING_CASTLE;
    } else if team == Teams::BLACK && (occupied_spaces.0 & BLACK_QUEENSIDE_CASTLE_BLOCKS) == 0 {
        castling_moves.0 |= BLACK_QUEENSIDE_KING_CASTLE;
    }

    castling_moves
}