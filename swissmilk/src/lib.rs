#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, reason = "Only allowed during development")]
#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::blanket_clippy_restriction_lints, clippy::arbitrary_source_item_ordering, clippy::implicit_return, clippy::missing_trait_methods, clippy::partial_pub_fields, clippy::unreachable, clippy::indexing_slicing, clippy::arithmetic_side_effects, clippy::std_instead_of_alloc, clippy::print_stdout, clippy::single_call_fn, clippy::expect_used, clippy::std_instead_of_core, clippy::separated_literal_suffix, clippy::mod_module_files, clippy::pub_use, clippy::default_numeric_fallback, clippy::as_conversions, clippy::integer_division_remainder_used, clippy::cast_possible_truncation, clippy::string_add, clippy::string_add_assign, clippy::multiple_inherent_impl, clippy::use_debug, clippy::question_mark_used, clippy::pattern_type_mismatch, clippy::redundant_pub_crate, clippy::pub_without_shorthand, clippy::else_if_without_else, clippy::renamed_function_params, clippy::integer_division, clippy::multiple_unsafe_ops_per_block, clippy::must_use_candidate, clippy::manual_non_exhaustive, clippy::module_name_repetitions,
    reason = "Toooooo pedantic"
)]
#![allow(
    clippy::missing_panics_doc, clippy::missing_docs_in_private_items, clippy::missing_errors_doc, clippy::missing_safety_doc,
    reason = "Don't wanna deal with docs for NOW"
)]

pub mod random;
pub mod chess_types;
pub mod pairing;
pub mod player;
pub mod tournament;
pub mod dutch;
pub mod pairing_system;
pub mod berger_table;
