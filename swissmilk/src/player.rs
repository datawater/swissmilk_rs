use crate::chess_types::{Color, ColorPreference, ColorPreferenceLevel, Title};
use crate::random::{self, NewRandom};
use crate::tournament::Tournament;

use std::cmp::Ordering;

pub type PlayerId = u64;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
#[expect(clippy::exhaustive_structs, reason = "Should never change")]
pub struct PlayerInfo {
    pub id: PlayerId,
    pub name: String,
    pub rating: u16,
    pub title: Title,
}

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
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
    #[inline]
    pub fn new(id: PlayerId, name: &str, title: Title, rating: u16) -> Self {
        Self {
            id,
            name: name.to_owned(),
            title,
            rating,
        }
    }

    #[inline]
    pub fn as_string_csv_like(&self) -> String {
        format!("{} {} {:?} {}", self.id, self.name, self.title, self.rating)
    }

    #[inline]
    pub const fn csv_header() -> &'static str {
        "id name title rating"
    }
}

impl PartialOrd for PlayerInfo {
    #[expect(clippy::non_canonical_partial_ord_impl, reason = "")]
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rating
            .partial_cmp(&other.rating)
            .map(|x| x.then(self.title.cmp(&other.title)))
            .map(|x| x.then(self.name.cmp(&other.name).reverse()))
    }
}

impl Ord for PlayerInfo {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("partial_cmp for PlayerInfo failed")
    }
}

impl Player {
    #[inline]
    pub fn new(id: PlayerId, name: &str, title: Title, rating: u16) -> Self {
        Self {
            info: Box::new(PlayerInfo::new(id, name, title, rating)),
            ..Default::default()
        }
    }

    #[expect(
        clippy::missing_inline_in_public_items,
        reason = "Too big of a function"
    )]
    pub fn get_dutch_color_preference(&self) -> ColorPreference {
        let chl = self.color_history.len();

        if self.color_difference.abs() > 1 {
            return match self.color_difference.signum() {
                -1 => ColorPreference::new_with_width(
                    Color::White,
                    ColorPreferenceLevel::Absolute,
                    self.color_difference.unsigned_abs(),
                ),
                1 => ColorPreference::new_with_width(
                    Color::Black,
                    ColorPreferenceLevel::Absolute,
                    self.color_difference.unsigned_abs(),
                ),
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

    #[inline]
    pub fn as_string_csv_like(&self) -> String {
        format!("{} {}", self.info.as_string_csv_like(), self.score)
    }

    #[inline]
    pub const fn csv_header() -> &'static str {
        "id name title rating score"
    }
}

const NAMES: [&str; 200] = include!("../../data/names.txt");

impl NewRandom for PlayerInfo {
    #[expect(
        clippy::missing_inline_in_public_items,
        reason = "Too big of a function"
    )]
    fn new_random() -> Self {
        let id = random::rand::<u64>();
        let name = NAMES[random::rand::<u32>() as usize % 200].to_owned()
            + &((random::rand::<u32>() % u32::from(u16::MAX)).to_string());
        let rating = random::rand::<u32>() as u16 % 2000 + 1000;
        let title = Title::try_from(random::rand::<u32>() as i32 % 9).expect("UNREACHABLE");

        Self {
            id,
            name,
            rating,
            title,
        }
    }
}

impl NewRandom for Player {
    #[inline]
    fn new_random() -> Self {
        Self {
            info: Box::new(PlayerInfo::new_random()),
            ..Default::default()
        }
    }
}

impl PartialOrd for Player {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score
            .partial_cmp(&other.score)
            .map(|x| x.then(self.pairing_number.cmp(&other.pairing_number)))
    }
}
