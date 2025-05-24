use std::collections::{BTreeMap, btree_map};
use std::error::Error;
use std::fmt::Write as _;

use itertools::Itertools as _;
use smallvec::{SmallVec, smallvec};

use crate::berger_table::BergerTablePairingSystem;
use crate::chess_types::ResultScoreConfig;
use crate::dutch::DutchPairingSystem;
use crate::pairing::Pairing;
use crate::pairing_system::{PairingSystemType, PairngSystem as _};
use crate::player::{Player, PlayerId};

#[derive(Debug, Default, PartialEq)]
pub struct Tournament {
    pairing_system: PairingSystemType,

    // Optimal number of rounds ~= log_2(number of players) + k
    number_of_rounds: u8,
    current_round: u8,

    players: BTreeMap<PlayerId, Player>,

    // Player, round withdrawned
    withdrawn_players: SmallVec<[(PlayerId, u8); 2]>,
    past_round_pairings: SmallVec<[Pairing; 13]>,

    result_scores: ResultScoreConfig,
}

pub type ScoreGroups = BTreeMap<u8, SmallVec<[PlayerId; 24]>>;

impl Tournament {
    #[inline]
    pub fn new(number_of_rounds: u8, pairing_system: PairingSystemType) -> Self {
        Self {
            pairing_system,
            number_of_rounds,
            result_scores: (2, 1, 0),
            ..Default::default()
        }
    }

    #[inline]
    pub fn new_with_result_scores(
        number_of_rounds: u8,
        pairing_system: PairingSystemType,
        result_scores: ResultScoreConfig,
    ) -> Self {
        Self {
            pairing_system,
            number_of_rounds,
            result_scores,
            ..Default::default()
        }
    }

    #[inline]
    pub fn new_with_players(
        number_of_rounds: u8,
        pairing_system: PairingSystemType,
        players: BTreeMap<PlayerId, Player>,
    ) -> Self {
        Self {
            pairing_system,
            number_of_rounds,
            players,
            result_scores: (2, 1, 0),
            ..Default::default()
        }
    }

    #[inline]
    pub fn add_players(&mut self, players: Vec<Player>) {
        for x in players {
            self.players.insert(x.info.id, x);
        }
    }

    #[inline]
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

    #[inline]
    pub const fn get_pairing_system_type(&self) -> PairingSystemType {
        self.pairing_system
    }

    #[expect(
        clippy::missing_inline_in_public_items,
        reason = "Too big of a function"
    )]
    pub fn pair(&mut self) -> Result<Vec<Pairing>, Box<dyn Error>> {
        if !self.has_started() {
            self.start();
        }

        match self.pairing_system {
            PairingSystemType::BergerTable => BergerTablePairingSystem::pair_round(self),
            PairingSystemType::ClassicSwiss => DutchPairingSystem::pair_round(self),
            PairingSystemType::DubovSwiss => todo!(),
            PairingSystemType::BursteinSwiss => todo!(),
            PairingSystemType::Lim => todo!(),
            PairingSystemType::None => todo!(),
        }
    }

    pub(crate) fn assign_pairing_numbers(&mut self) {
        let mut pairing_number = self.player_count() as u16;

        for (_, player) in self
            .players
            .iter_mut()
            .sorted_by_key(|(_, player)| player.info.clone())
        {
            player.pairing_number = pairing_number;
            pairing_number -= 1;
        }
    }

    #[expect(
        clippy::missing_inline_in_public_items,
        reason = "Too big of a function"
    )]
    pub fn get_score_groups(&self) -> ScoreGroups {
        let mut scoregroup = ScoreGroups::new();

        self.players.values().for_each(|player| {
            if let btree_map::Entry::Vacant(entry) = scoregroup.entry(player.score) {
                entry.insert(smallvec![player.info.id]);
            } else {
                // SAFETY: We check if the key is present or not
                unsafe { scoregroup.get_mut(&player.score).unwrap_unchecked() }
                    .push(player.info.id);
            }
        });

        scoregroup
    }

    #[inline]
    pub const fn current_round(&self) -> u8 {
        self.current_round
    }

    #[inline]
    pub const fn bump_round(&mut self) {
        self.current_round += 1;
    }

    #[inline]
    pub fn start(&mut self) {
        self.assign_pairing_numbers();
        self.current_round = 1;
    }

    #[inline]
    pub const fn has_started(&self) -> bool {
        self.current_round > 0
    }

    #[inline]
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    #[inline]
    pub const fn get_players(&self) -> &BTreeMap<PlayerId, Player> {
        &self.players
    }

    #[inline]
    pub const fn get_players_mut(&mut self) -> &mut BTreeMap<PlayerId, Player> {
        &mut self.players
    }

    // I should probably put this in a trait
    #[expect(
        clippy::missing_inline_in_public_items,
        reason = "Too big of a function"
    )]
    pub fn as_string_csv_like(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buffer = String::new();
        buffer.write_fmt(format_args!(
            "Tournament\n\nNumber of rounds: {}\nCurrent round: {}\nWin/Draw/Lose scores: {:?}\n",
            self.number_of_rounds, self.current_round, self.result_scores
        ))?;

        buffer.write_fmt(format_args!(
            "\nWithdrawn players (count {}):\n{} round_widthdrawn\n",
            self.withdrawn_players.len(),
            Player::csv_header()
        ))?;

        for player in &self.withdrawn_players {
            buffer.write_fmt(format_args!(
                "{} {}\n",
                self.get_player(player.0)
                    .ok_or("UNREACHABLE")?
                    .as_string_csv_like(),
                player.1
            ))?;
        }

        if !self.has_started() {
            self.assign_pairing_numbers();
        }

        buffer.write_fmt(format_args!("\nPlayers:\n{}\n", Player::csv_header()))?;
        for player in self
            .players
            .iter()
            .sorted_by_key(|x| x.1.pairing_number)
            .rev()
        {
            buffer.write_fmt(format_args!("{}\n", player.1.as_string_csv_like(),))?;
        }

        Ok(buffer)
    }
}
