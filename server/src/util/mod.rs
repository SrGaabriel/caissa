use crate::game::{Coordinates, Move, PieceType};

pub fn parse_square_name(square_name: &str) -> Coordinates {
    let chars: Vec<char> = square_name.chars().collect();
    let x = chars[0] as usize - 'a' as usize + 1;
    let y = chars[1..].iter().collect::<String>().parse().unwrap();
    Coordinates { x: x as i8, y }
}

// Here we'll check things like mov.check, mov.piece, etc...
// Ideally, we'll have something like Nxf4+ for a knight capturing a piece at f4 with a check
// or O-O for a king side castle
// or e4 for a pawn moving to e4
// or exd5 for a pawn capturing a piece at d5
// or e8=Q for a pawn promoting to a queen
// or e8=Q+ for a pawn promoting to a queen with a check
// or e8=Q# for a pawn promoting to a queen with a checkmate
pub fn nameMove(mov: &Move) -> String {
    let mut result = String::new();
    let piece = mov.piece.piece_type;
    if piece != PieceType::Pawn {
        result.push(get_piece_letter(piece));
    }
    if mov.capture {
        result.push('x');
    }
    result.push_str(&format!("{}{}", (mov.to.x as u8 + 'a' as u8 - 1) as char, mov.to.y));
    if mov.promotion {
        result.push('=');
        // result.push(mov.piece.piece_type.to_ascii_uppercase());
    }
    if mov.check {
        result.push('+');
    } else if mov.ending == Some(crate::game::GameEnding::Checkmate) {
        result.push('#');
    }
    result
}

pub fn get_piece_letter(piece_type: PieceType) -> char {
    match piece_type {
        PieceType::Pawn => 'P',
        PieceType::Knight => 'N',
        PieceType::Bishop => 'B',
        PieceType::Rook => 'R',
        PieceType::Queen => 'Q',
        PieceType::King => 'K',
    }
}