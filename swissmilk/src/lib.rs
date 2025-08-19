#![warn(clippy::pedantic, clippy::missing_const_for_fn)]
#![deny(clippy::perf)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    incomplete_features,
    long_running_const_eval
)]

pub mod random;
pub mod chess_types;
pub mod pairing;
pub mod player;
pub mod tournament;
pub mod dutch;
pub mod pairing_system;
pub mod berger_table;
mod utils;