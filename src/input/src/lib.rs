// LINT-REPLACE-START
// This section is autogenerated, do not modify directly
// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![cfg_attr(allow_unknown_lints, allow(renamed_and_removed_lints))]
// enable all rustc's built-in lints
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	rust_2021_incompatible_closure_captures,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
	rust_2021_prelude_collisions,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unstable_features,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![cfg_attr(include_nightly_lints, deny(clippy::nursery))]
#![allow(
	clippy::blanket_clippy_restriction_lints,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::implicit_return,
	clippy::indexing_slicing,
	clippy::integer_arithmetic,
	clippy::missing_docs_in_private_items,
	clippy::mod_module_files,
	clippy::module_name_repetitions,
	clippy::option_if_let_else,
	clippy::pub_use,
	clippy::redundant_pub_crate,
	clippy::tabs_in_doc_comments,
	clippy::too_many_lines
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
#![cfg_attr(
	include_nightly_lints,
	allow(
		clippy::arithmetic_side_effects,
		clippy::bool_to_int_with_if,
		clippy::std_instead_of_alloc,
		clippy::std_instead_of_core
	)
)]
// LINT-REPLACE-END

//! Git Interactive Rebase Tool - Input Module
//!
//! # Description
//! This module is used to handle working with input events.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod custom_event;
mod custom_key_binding;
mod event;
mod event_handler;
mod event_provider;
mod input_options;
mod key_bindings;
mod key_event;
mod standard_event;
#[cfg(not(tarpaulin_include))]
pub mod testutil;
mod thread;

pub use crossterm::event::{Event as RawEvent, KeyCode, KeyModifiers, MouseEvent, MouseEventKind};

pub use self::{
	custom_event::CustomEvent,
	custom_key_binding::CustomKeybinding,
	event::Event,
	event_handler::EventHandler,
	event_provider::{read_event, EventReaderFn},
	input_options::InputOptions,
	key_bindings::{map_keybindings, KeyBindings},
	key_event::KeyEvent,
	standard_event::StandardEvent,
	thread::{State, Thread, THREAD_NAME},
};
