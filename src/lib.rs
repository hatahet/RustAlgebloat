// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#[feature(globs, macro_rules)];

#[crate_type="lib"];
#[crate_id="algebloat"];

pub use matrix::*;
pub use vector::*;

pub mod matrix;
pub mod vector;
