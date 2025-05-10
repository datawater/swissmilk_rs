#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, reason = "Only allowed during development")]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::blanket_clippy_restriction_lints, clippy::missing_docs_in_private_items, clippy::arbitrary_source_item_ordering, clippy::implicit_return, clippy::missing_trait_methods, clippy::partial_pub_fields, clippy::unreachable, clippy::indexing_slicing, clippy::arithmetic_side_effects, clippy::std_instead_of_alloc, clippy::print_stdout, clippy::single_call_fn, clippy::expect_used,
    reason = "Toooooo pedantic"
)]

mod chess_types;
mod pairing;
mod player;
mod tournament;

fn main() {
    println!("Hello, world!");
}
