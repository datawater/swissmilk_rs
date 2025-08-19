use crate::chess_types::Color;
use crate::player::PlayerId;
use crate::tournament::Tournament;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pairing {
    pub left: PlayerId,
    pub right: Option<PlayerId>,

    pub color_left: Color,
    pub color_right: Color,
}

impl Pairing {
    #[inline]
    pub const fn new(
        left: PlayerId,
        right: Option<PlayerId>,
        color_left: Color,
        color_right: Color,
    ) -> Self {
        Self {
            left,
            right,
            color_left,
            color_right,
        }
    }

    #[inline]
    pub fn get_score_difference(&self, tournament: &Tournament) -> u8 {
        if self.right.is_none() {
            return 0;
        }

        let left = tournament
            .get_player(self.left)
            .expect("Invalid player id/No player found");

        // SAFETY: Already had returned if is none
        let right = tournament
            .get_player(unsafe { self.right.unwrap_unchecked() })
            .expect("Invalid player id/No player found");

        (i16::from(left.score) - i16::from(right.score)).unsigned_abs() as u8
    }
}
