// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - Core Language Implementation
// Harvard Architecture: Control (Turing-complete) + Data (Total)

pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod number;
pub mod error;
pub mod typechecker;
pub mod purity;
pub mod reversible;
pub mod wasm;

pub use ast::*;
pub use parser::*;
pub use interpreter::*;
pub use number::*;
pub use error::*;
pub use typechecker::*;
pub use purity::*;
pub use reversible::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
