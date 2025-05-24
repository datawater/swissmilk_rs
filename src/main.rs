use std::error::Error;

use swissmilk::pairing_system::PairingSystemType;
use swissmilk::player::Player;
use swissmilk::random::NewRandom as _;
use swissmilk::tournament::Tournament;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tournament = Tournament::new(8, PairingSystemType::BergerTable);
    for _ in 0..9 {
        tournament.add_player(Player::new_random());
    }

    println!("{}", tournament.as_string_csv_like()?);

    let pairings = tournament.pair()?;
    for pair in pairings {
        println!(
            "{:?} VS {:?}. ({:?} vs {:?})",
            pair.left, pair.right, pair.color_left, pair.color_right
        );
    }

    Ok(())
}
