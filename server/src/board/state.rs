use crate::board::{Team, Teams};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct ChessState {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<usize>,
    pub team_to_play: Team
}

pub struct CastlingSides;
impl CastlingSides {
    pub const KINGSIDE: usize = 0;
    pub const QUEENSIDE: usize = 1;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct CastlingRights(u8);

impl CastlingRights {
    const NONE: u8 = 0b0000;
    const WHITE_KINGSIDE: u8 = 0b0001;
    const WHITE_QUEENSIDE: u8 = 0b0010;
    const BLACK_KINGSIDE: u8 = 0b0100;
    const BLACK_QUEENSIDE: u8 = 0b1000;
    const ALL: u8 = Self::WHITE_KINGSIDE | Self::WHITE_QUEENSIDE | Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE;

    pub fn none() -> Self {
        CastlingRights(CastlingRights::NONE)
    }
    pub fn all() -> Self {
        CastlingRights(CastlingRights::ALL)
    }

    pub fn allow(&mut self, team: Team, side: usize) {
        self.0 |= self.get_bit(team, side);
    }

    pub fn disallow(&mut self, team: Team, side: usize) {
        self.0 &= !self.get_bit(team, side);
    }

    pub fn is_allowed(&self, team: Team, side: usize) -> bool {
        self.0 & self.get_bit(team, side) != 0
    }

    pub fn disallow_all(&mut self, team: Team) {
        let bits = match team {
            Teams::WHITE => Self::WHITE_KINGSIDE | Self::WHITE_KINGSIDE,
            Teams::BLACK => Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE,
            _ => panic!("Invalid team")
        };
        self.0 &=  !bits;
    }

    pub fn get_bit(&self, team: Team, side: usize) -> u8 {
        match (team, side) {
            (Teams::WHITE, CastlingSides::KINGSIDE) => Self::WHITE_KINGSIDE,
            (Teams::WHITE, CastlingSides::QUEENSIDE) => Self::WHITE_QUEENSIDE,
            (Teams::BLACK, CastlingSides::KINGSIDE) => Self::BLACK_KINGSIDE,
            (Teams::BLACK, CastlingSides::QUEENSIDE) => Self::BLACK_QUEENSIDE,
            _ => 0,
        }
    }
}


