// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// JtV-Specific Library - Functions unique to Julia the Viper
// These functions leverage JtV's unique features:
// - Harvard Architecture (Control/Data separation)
// - Reversible computing
// - 7 number systems
// - Purity guarantees

pub mod number_systems;
pub mod reversible;

pub use number_systems::*;
pub use reversible::*;
