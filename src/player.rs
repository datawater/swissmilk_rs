use crate::chess_types::{Color, ColorPreference, ColorPreferenceLevel, Title};
use crate::tournament::Tournament;

use core::cmp::Ordering;

pub type PlayerId = u64;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct PlayerInfo {
    pub id: PlayerId,
    pub name: String,
    pub rating: u16,
    pub title: Title,
}

#[derive(Debug, Default, PartialEq)]
pub struct Player {
    belongs_to_tournament: Box<Tournament>,

    pub info: Box<PlayerInfo>,

    pub score: u8,
    pub previous_opponents: Vec<PlayerId>,
    pub color_history: Vec<Color>,
    pub color_difference: i8,

    pub has_bye: bool,
    pub pairing_number: u16,
    pub downfloats: u8,
    pub upfloats: u8,

    pub history: Vec<PlayerId>,
}

impl PlayerInfo {
    pub fn new(id: PlayerId, name: &str, title: Title, rating: u16) -> Self {
        Self {
            id,
            name: name.to_owned(),
            title,
            rating,
        }
    }
}

impl PartialOrd for PlayerInfo {
    #[expect(clippy::non_canonical_partial_ord_impl, reason = "")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rating
            .partial_cmp(&other.rating)
            .map(|x| x.then(self.title.cmp(&other.title)))
            .map(|x| x.then(self.name.cmp(&other.name)))
    }
}

impl Ord for PlayerInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("partial_cmp for PlayerInfo failed")
    }
}

impl Player {
    pub fn new(id: PlayerId, name: &str, title: Title, rating: u16) -> Self {
        Self {
            info: Box::new(PlayerInfo::new(id, name, title, rating)),
            ..Default::default()
        }
    }

    pub fn get_color_preference(&self) -> ColorPreference {
        let chl = self.color_history.len();

        if self.color_difference.abs() > 1 {
            return match self.color_difference.signum() {
                -1 => ColorPreference::new(Color::White, ColorPreferenceLevel::Absolute),
                1 => ColorPreference::new(Color::Black, ColorPreferenceLevel::Absolute),
                _ => unreachable!(),
            };
        }

        if chl > 1 && self.color_history[chl - 1] == self.color_history[chl - 2] {
            return match self.color_history[chl - 1] {
                Color::White => ColorPreference::new(Color::Black, ColorPreferenceLevel::Absolute),
                Color::Black => ColorPreference::new(Color::White, ColorPreferenceLevel::Absolute),
                Color::None => unreachable!(),
            };
        }

        if self.color_difference.abs() == 1 {
            return match self.color_difference.signum() {
                -1 => ColorPreference::new(Color::White, ColorPreferenceLevel::Strong),
                1 => ColorPreference::new(Color::Black, ColorPreferenceLevel::Strong),
                _ => unreachable!(),
            };
        }

        if chl > 0 && self.color_difference == 0 {
            return ColorPreference::new(!self.color_history[chl - 1], ColorPreferenceLevel::Mild);
        }

        ColorPreference::new(Color::None, ColorPreferenceLevel::None)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score
            .partial_cmp(&other.score)
            .map(|x| x.then(self.pairing_number.cmp(&other.pairing_number)))
    }
}
