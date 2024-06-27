use crate::board::BitBoard;

pub fn properly_mask_all_bishop_moves(bishops: &BitBoard, blocks: &BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);
    let mut bitboard = bishops.0;

    while bitboard != 0 {
        let bishop = 1 << bitboard.trailing_zeros();
        bitboard ^= bishop;

        let attacks = properly_mask_bishop_moves(bishop, blocks);
        moves.0 |= attacks.0;
    }
    moves
}

// Now we'll consider blocking pieces
pub fn properly_mask_bishop_moves(bishop_bit: u64, block: &BitBoard) -> BitBoard {
    let mut attacks = 0;
    let square = bishop_bit.trailing_zeros();
    let (r, f) = (square / 8, square % 8);

    for (r, f) in ((r + 1)..=7).zip((f + 1)..=7) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in ((r + 1)..=7).zip((0..f).rev()) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in (0..r).rev().zip((f + 1)..=7) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in (0..r).rev().zip((0..f).rev()) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }
    BitBoard(attacks)
}

pub fn properly_mask_all_rook_moves(rooks: &BitBoard, blocks: &BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);
    let mut bitboard = rooks.0;

    while bitboard != 0 {
        let rook = 1 << bitboard.trailing_zeros();
        bitboard ^= rook;

        let attacks = properly_mask_rook_moves(rook, blocks);
        moves.0 |= attacks.0;
    }
    moves
}

pub fn properly_mask_rook_moves(rook_bit: u64, block: &BitBoard) -> BitBoard {
    let mut attacks = 0;
    let square = rook_bit.trailing_zeros();
    let (r, f) = (square / 8, square % 8);

    for r in (r + 1)..=7 {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for r in (0..r).rev() {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for f in (f + 1)..=7 {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for f in (0..f).rev() {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }
    BitBoard(attacks)
}

pub fn properly_mask_all_queen_moves(queens: &BitBoard, blocks: &BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);
    let mut bitboard = queens.0;

    while bitboard != 0 {
        let queen = 1 << bitboard.trailing_zeros();
        bitboard ^= queen;

        let attacks = properly_mask_queen_moves(queen, blocks);
        moves.0 |= attacks.0;
    }
    moves
}

pub fn properly_mask_queen_moves(queen_bit: u64, block: &BitBoard) -> BitBoard {
    let mut attacks = 0;
    let square = queen_bit.trailing_zeros();
    let (r, f) = (square / 8, square % 8);

    for r in (r + 1)..=7 {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for r in (0..r).rev() {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for f in (f + 1)..=7 {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for f in (0..f).rev() {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }
    for (r, f) in ((r + 1)..=7).zip((f + 1)..=7) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in ((r + 1)..=7).zip((0..f).rev()) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in (0..r).rev().zip((f + 1)..=7) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    for (r, f) in (0..r).rev().zip((0..f).rev()) {
        attacks |= 1 << (r * 8 + f);
        if block.0 & (1 << (r * 8 + f)) != 0 { break; }
    }

    BitBoard(attacks)
}

pub fn mask_bishop_attacks(bishop_bit: u64) -> BitBoard {
    let mut attacks = 0;
    let (r, f) = (bishop_bit / 8, bishop_bit % 8);

    for (r, f) in (r..=7).zip(f..=7) { attacks |= 1 << (r * 8 + f); }
    for (r, f) in (r..=7).zip((0..=f).rev()) { attacks |= 1 << (r * 8 + f); }
    for (r, f) in (0..=r).rev().zip(f..=7) { attacks |= 1 << (r * 8 + f); }
    for (r, f) in (0..=r).rev().zip((0..=f).rev()) { attacks |= 1 << (r * 8 + f); }

    BitBoard(attacks)
}

pub fn mask_rook_attacks(rook_bit: u64) -> BitBoard {
    let mut attacks = 0;
    let (r, f) = (rook_bit / 8, rook_bit % 8);

    for r in r..=7 { attacks |= 1 << (r * 8 + f); }
    for r in (0..=r).rev() { attacks |= 1 << (r * 8 + f); }
    for f in r..=7 { attacks |= 1 << (r * 8 + f); }
    for f in (0..=f).rev() { attacks |= 1 << (r * 8 + f); }

    BitBoard(attacks)
}
