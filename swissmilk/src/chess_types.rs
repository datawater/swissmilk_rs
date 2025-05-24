use crate::back_to_enum;
use std::ops::Not;

back_to_enum! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
    #[repr(u8)]
    #[expect(clippy::exhaustive_enums, reason = "Should never change")]
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
}

back_to_enum! {
    #[expect(clippy::exhaustive_enums, reason = "Should never change")]
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
    pub enum Color {
        #[default]
        None,
        White,
        Black,
    }
}

back_to_enum! {
    #[expect(clippy::exhaustive_enums, reason = "Should never change")]
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
    pub enum ColorPreferenceLevel {
        #[default]
        None,
        Mild,
        Strong,
        Absolute,
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ColorPreference {
    color: Color,
    level: ColorPreferenceLevel,
    width: u8,
}

impl Not for Color {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::None => self,
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl ColorPreference {
    #[inline]
    pub const fn new(color: Color, level: ColorPreferenceLevel) -> Self {
        Self {
            color,
            level,
            width: 0,
        }
    }

    #[inline]
    pub const fn new_with_width(color: Color, level: ColorPreferenceLevel, width: u8) -> Self {
        Self {
            color,
            level,
            width,
        }
    }
}

// Using u8 instead of f32 is so extra, but whatever
// win score, draw score, lose score
pub type ResultScoreConfig = (u8, u8, u8);
