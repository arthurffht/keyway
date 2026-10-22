// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pure Rust core of Keyway: normalization, query parsing, matching, scoring,
//! frecency and the result model.
//!
//! This crate must not depend on GLib (docs/PLAN.md §10.4).

pub mod normalize;
pub mod query;
