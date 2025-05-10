use core::ops::Not;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(u8)]
#[expect(clippy::upper_case_acronyms, reason = "Official names from FIDE")]
pub enum Title {
    #[default]
    None,
    WCM,
    WFM,
    CM,
    WIM,
    FM,
    WGM,
    IM,
    GM,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    None,
    White,
    Black,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ColorPreferenceLevel {
    #[default]
    None,
    Mild,
    Strong,
    Absolute,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ColorPreference {
    color: Color,
    level: ColorPreferenceLevel,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::None => self,
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl ColorPreference {
    pub const fn new(color: Color, level: ColorPreferenceLevel) -> Self {
        Self { color, level }
    }
}

// Using u8 instead of f32 is so extra, but whatever
// win score, draw score, lose score
pub type ResultScoreConfig = (u8, u8, u8);
