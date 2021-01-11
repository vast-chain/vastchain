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

//! Tree route info type definition

use vast_types::H256;

/// Represents a tree route between `from` block and `to` block:
#[derive(Debug)]
pub struct TreeRoute {
	/// A vector of hashes of all blocks, ordered from `from` to `to`.
	pub blocks: Vec<H256>,
	/// Best common ancestor of these blocks.
	pub ancestor: H256,
	/// An index where best common ancestor would be.
	pub index: usize,
	/// Whvaster it has finalized blocks from `from` (inclusive) to `ancestor` (exclusive).
	pub is_from_route_finalized: bool,
}
