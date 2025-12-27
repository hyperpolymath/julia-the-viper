// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Core Language Implementation
// Harvard Architecture: Control (Turing-complete) + Data (Total)

// Allow some clippy lints that require significant refactoring
#![allow(clippy::should_implement_trait)]
#![allow(clippy::while_let_on_iterator)]
#![allow(clippy::manual_strip)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]

pub mod ast;
pub mod error;
pub mod formatter;
pub mod interpreter;
pub mod number;
pub mod parser;
pub mod purity;
pub mod reversible;
pub mod typechecker;
pub mod wasm;

pub use ast::*;
pub use error::*;
pub use formatter::*;
pub use interpreter::*;
pub use number::*;
pub use parser::*;
pub use purity::*;
pub use reversible::*;
pub use typechecker::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
