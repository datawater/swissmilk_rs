use crate::chess_types::Color;
use crate::player::PlayerId;
use crate::tournament::Tournament;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pairing {
    left: PlayerId,
    right: PlayerId,

    color_left: Color,
    color_right: Color,
}

impl Pairing {
    pub const fn new(
        left: PlayerId,
        right: PlayerId,
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

    #[expect(
        clippy::cast_possible_truncation,
        clippy::as_conversions,
        reason = "Difference is never above 255"
    )]
    pub fn get_score_difference(&self, tournament: &Tournament) -> u8 {
        let left = tournament
            .get_player(self.left)
            .expect("Invalid player id/No player found");
        let right = tournament
            .get_player(self.right)
            .expect("Invalid player id/No player found");

        (i16::from(left.score) - i16::from(right.score)).unsigned_abs() as u8
    }
}
