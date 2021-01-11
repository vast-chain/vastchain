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

//! Test structures for JSON deserialization.

/// Blockchain test helpers
pub mod blockchain;
/// Difficulty test helpers
pub mod difficulty;
/// Tests to skip helpers
pub mod skip;
/// State test helpers
pub mod state;
/// Test primitives
pub mod tester;
/// Transaction test helpers
pub mod transaction;
/// Trie test helpers
pub mod trie;
/// Vm test helpers
pub mod vm {
	/// Type for running `vm` tests
	pub type Test = super::tester::GenericTester<String, crate::vm::Vm>;
}
