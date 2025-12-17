// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper Library System
//
// This module organizes the standard library into:
// - common/  : Language-agnostic functions (math, collections)
// - jtv/     : JtV-specific functions (number systems, reversible computing)
//
// The common library contains universal functions that could be shared
// across language implementations. The jtv library contains functions
// that leverage JtV's unique features.

pub mod common;
pub mod jtv;

// Re-export for convenience
pub use common::{math, collections};
pub use jtv::{number_systems, reversible};
