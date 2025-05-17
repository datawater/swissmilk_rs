#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, reason = "Only allowed during development")]
#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::blanket_clippy_restriction_lints, clippy::missing_docs_in_private_items, clippy::arbitrary_source_item_ordering, clippy::implicit_return, clippy::missing_trait_methods, clippy::partial_pub_fields, clippy::unreachable, clippy::indexing_slicing, clippy::arithmetic_side_effects, clippy::std_instead_of_alloc, clippy::print_stdout, clippy::single_call_fn, clippy::expect_used, clippy::std_instead_of_core, clippy::separated_literal_suffix, clippy::mod_module_files, clippy::pub_use, clippy::default_numeric_fallback, clippy::as_conversions, clippy::integer_division_remainder_used, clippy::cast_possible_truncation, clippy::string_add, clippy::string_add_assign, clippy::multiple_inherent_impl, clippy::use_debug, clippy::question_mark_used, clippy::pattern_type_mismatch, clippy::redundant_pub_crate, clippy::pub_without_shorthand, clippy::else_if_without_else, clippy::renamed_function_params, clippy::integer_division, clippy::multiple_unsafe_ops_per_block,
    reason = "Toooooo pedantic"
)]

use std::error::Error;

use swissmilk::pairing_system::PairingSystemType;
use swissmilk::player::Player;
use swissmilk::tournament::Tournament;
use swissmilk::random::NewRandom as _;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tournament = Tournament::new(8, PairingSystemType::BergerTable);
    for _ in 0..9 {
        tournament.add_player(Player::new_random());
    }

    println!("{}", tournament.as_string_csv_like()?);

    let pairings = tournament.pair()?;
    for pair in pairings {
        println!("{:?} VS {:?}. ({:?} vs {:?})", pair.left, pair.right, pair.color_left, pair.color_right);
    }

    Ok(())
}
