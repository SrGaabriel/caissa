use std::fmt::Display;

use crate::board::PossibleMove;
use crate::game::Vector;

pub fn square_to_vector(square: &str) -> Vector {
    let mut chars = square.chars();
    let file = chars.next().unwrap();
    let rank = chars.next().unwrap();
    let file = file as u8 - 'a' as u8;
    let rank = rank as u8 - '1' as u8;
    Vector::new(rank, file)
}

impl Display for PossibleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let from = format!("{}{}", (self.origin.x + 1) as char, (self.origin.y + 1) as char);
        let to = format!("{}{}", (self.target.x + 1) as char, (self.target.y + 1) as char);
        result.push_str(&from);
        result.push_str(&to);
        write!(f, "{}", result)
    }
}