use std::error::Error;

use crate::{pairing::Pairing, tournament::Tournament};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum PairingSystemType {
    #[default]
    BergerTable,
    ClassicSwiss,
    DubovSwiss,
    BursteinSwiss,
    Lim,

    None,
}

pub(crate) trait PairngSystem {
    fn pair_round(tournament: &mut Tournament) -> Result<Vec<Pairing>, Box<dyn Error>>;
}
