// Copyright 2015-2020 Vast Technologies (UK) Ltd.
// This file is part of Vast.

// Vast is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Vast is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Vast.  If not, see <http://www.gnu.org/licenses/>.

//! This crate provides a state machine and the facilities needed to execute transactions and the
//! code contained therein, as well as contract based transaction permissions. All vast
//! engines embed a `Machine`.

pub mod executed;
pub mod executed_block;
pub mod executive;
pub mod externalities;
pub mod machine;
pub mod substate;
pub mod transaction_ext;
pub mod tx_filter;

pub use crate::{
	executed_block::ExecutedBlock,
	machine::Machine
};

#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
