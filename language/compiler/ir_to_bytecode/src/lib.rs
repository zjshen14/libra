// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![feature(iter_copied)]

#[macro_use]
extern crate log;

pub mod compiler;
pub mod errors;
pub mod parser;

// Unit tests for this crate are in the parent "compiler" crate.
