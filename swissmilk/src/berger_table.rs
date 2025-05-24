use std::error::Error;
use std::mem;

use itertools::Itertools as _;

use crate::chess_types::Color;
use crate::pairing::Pairing;
use crate::pairing_system::PairngSystem;
use crate::player::Player;
use crate::tournament::Tournament;

pub(crate) struct BergerTablePairingSystem;

impl PairngSystem for BergerTablePairingSystem {
    // TODO: Tests for BergerTablePairingSystem::pair_round
    fn pair_round(tournament: &mut Tournament) -> Result<Vec<Pairing>, Box<dyn Error>> {
        if tournament.has_started() {
            tournament.start();
        }

        let round = tournament.current_round();

        // PERF: Collect shouldn't be that slow because references
        let mut players = tournament
            .get_players()
            .iter()
            .sorted_by_key(|(_id, player)| player.pairing_number)
            .collect::<Vec<_>>();
        let mut player_count = tournament.player_count();

        let bye_player = Player::default();
        if player_count % 2 != 0 {
            player_count += 1;
            players.push((&0, &bye_player));
        }

        let fixed = players[0];
        let mut rest = players[1..].to_vec();

        let k = usize::from(round - 1) % (player_count - 1);
        rest.rotate_left(k);

        let mut full = Vec::with_capacity(player_count);
        full.push(fixed);
        full.extend(rest);

        drop(players);

        let mut pairings = Vec::with_capacity(player_count >> 1);
        for i in 0..(player_count >> 1) {
            let (mut left, mut right) = (Some(full[i]), Some(full[player_count - 1 - i]));

            let mut left_color = Color::White;
            let mut right_color = Color::Black;

            if round % 2 == 0 {
                mem::swap(&mut left_color, &mut right_color);
            }

            // SAFETY: Always Some
            unsafe {
                if *left.unwrap_unchecked().0 == 0 {
                    mem::swap(&mut left, &mut right);

                    right = None;
                    right_color = Color::None;
                } else if *right.unwrap_unchecked().0 == 0 {
                    right = None;
                    right_color = Color::None;
                }
            }

            pairings.push(Pairing::new(
                // SAFETY: Always Some
                unsafe { *left.unwrap_unchecked().0 },
                right.map(|x| *x.0),
                left_color,
                right_color,
            ));
        }

        Ok(pairings)
    }
}
