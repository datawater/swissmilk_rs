use std::error::Error;

use itertools::Itertools as _;
use num::FromPrimitive as _;

use crate::chess_types::Color;
use crate::pairing::Pairing;
use crate::pairing_system::PairngSystem;
use crate::tournament::Tournament;

pub(in crate) struct DutchPairingSystem;

impl PairngSystem for DutchPairingSystem {
    fn pair_round(tournament: &mut Tournament) -> Result<Vec<Pairing>, Box<dyn Error>> {
        if !tournament.has_started() {
            tournament.start();
        }

        if tournament.current_round() == 1 {
            return Self::pair_first_round(tournament);
        }

        todo!()
    }
}

impl DutchPairingSystem {
    fn pair_first_round(tournament: &Tournament) -> Result<Vec<Pairing>, Box<dyn Error>> {
        let binding = tournament
            .get_players()
            .iter()
            .sorted_by_key(|(_id, player)| u16::MAX - player.pairing_number);
        let players = binding.as_slice();
        let mut pairings = Vec::with_capacity(tournament.player_count() >> 1);

        let mut is_odd = false;
        let mut player_count = tournament.player_count();

        if player_count % 2 == 1 {
            is_odd = true;
            player_count -= 1;
        }

        for i in 0..player_count >> 1 {
            let color_left = Color::from_u8((player_count % 2) as u8 + 1).ok_or("UNREACHABLE")?;

            pairings.push(Pairing::new(
                *players[i].0,
                Some(*players[player_count - 1 - i].0),
                color_left,
                !color_left,
            ));
        }

        if is_odd {
            pairings.push(Pairing::new(
                *players[player_count].0,
                None,
                Color::None,
                Color::None,
            ));
        }

        Ok(pairings)
    }
}
