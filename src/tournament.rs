use std::collections::{BTreeMap, btree_map};

use itertools::Itertools as _;

use crate::{
    chess_types::ResultScoreConfig,
    pairing::Pairing,
    player::{Player, PlayerId},
};

#[derive(Debug, Default, PartialEq)]
pub struct Tournament {
    // Optimal number of rounds ~= log_2(number of players) + k
    number_of_rounds: u8,
    current_round: u8,

    players: BTreeMap<PlayerId, Player>,

    // Player, round withdrawned
    withdrawn_players: Vec<(PlayerId, u8)>,
    past_round_pairings: Vec<Pairing>,

    result_scores: ResultScoreConfig,
}

pub type ScoreGroups = BTreeMap<u8, Vec<PlayerId>>;

impl Tournament {
    pub fn new(number_of_rounds: u8) -> Self {
        Self {
            number_of_rounds,
            result_scores: (2, 1, 0),
            ..Default::default()
        }
    }

    pub fn new_with_result_scores(number_of_rounds: u8, result_scores: ResultScoreConfig) -> Self {
        Self {
            number_of_rounds,
            result_scores,
            ..Default::default()
        }
    }

    pub fn new_with_players(number_of_rounds: u8, players: BTreeMap<PlayerId, Player>) -> Self {
        Self {
            number_of_rounds,
            players,
            result_scores: (2, 1, 0),
            ..Default::default()
        }
    }

    pub fn add_players(&mut self, players: Vec<Player>) {
        for x in players {
            self.players.insert(x.info.id, x);
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.info.id, player);
    }

    #[inline]
    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    #[inline]
    pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
    }

    #[expect(
        clippy::pattern_type_mismatch,
        reason = "Can't figure out how to refactor"
    )]
    pub fn assign_pairing_numbers(&mut self) {
        let mut pairing_number = 1;

        for (_, player) in self
            .players
            .iter_mut()
            .sorted_by_key(|(_, player)| player.info.clone())
        {
            player.pairing_number = pairing_number;
            pairing_number += 1;
        }
    }

    pub fn get_score_groups(&self) -> ScoreGroups {
        let mut scoregroup = ScoreGroups::new();

        self.players.values().for_each(|player| {
            if let btree_map::Entry::Vacant(entry) = scoregroup.entry(player.score) {
                entry.insert(vec![player.info.id]);
            } else {
                // SAFETY: We check if the key is present or not
                unsafe { scoregroup.get_mut(&player.score).unwrap_unchecked() }
                    .push(player.info.id);
            }
        });

        scoregroup
    }
}
