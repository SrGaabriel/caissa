use crate::game::Vector;

pub fn square_to_vector(square: &str) -> Vector {
    let mut chars = square.chars();
    let file = chars.next().unwrap();
    let rank = chars.next().unwrap();
    let file = file as u8 - 'a' as u8;
    let rank = rank as u8 - '1' as u8;
    Vector::new(rank as i32, file as i32)
}